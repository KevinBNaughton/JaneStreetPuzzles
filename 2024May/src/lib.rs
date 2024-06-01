use {
    dioxus::prelude::*,
    std::{
        collections::{HashMap, HashSet},
        iter::successors,
    },
    uuid::Uuid,
};

pub mod palindrome;
pub mod primes_27293;
pub mod rows;

pub const ROWS: usize = 11;
pub const COLS: usize = 11;
pub static GROUP_NUMBERS: &[usize] = &[
    0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 0, 4, 4, 4, 1, 1, 2, 3, 3, 3, 5, 0, 4, 4, 1, 1, 1, 2, 3, 3, 3,
    5, 0, 4, 4, 1, 1, 6, 6, 3, 5, 5, 5, 0, 4, 1, 1, 3, 3, 6, 3, 5, 7, 5, 0, 3, 3, 3, 3, 3, 3, 3, 7,
    7, 8, 9, 3, 3, 3, 3, 10, 10, 3, 7, 7, 7, 9, 9, 11, 3, 11, 10, 10, 3, 3, 7, 3, 9, 9, 11, 11, 11,
    10, 10, 3, 3, 3, 3, 9, 11, 11, 9, 9, 9, 10, 3, 3, 3, 12, 9, 9, 9, 9, 9, 10, 10, 10, 3, 3, 12,
];

lazy_static::lazy_static! {
    pub static ref LOW_PRIMES: Vec<f64> = vec![
        2.0,
        3.0,
        5.0,
        7.0,
        11.0,
        13.0,
        17.0,
        19.0,
        23.0,
        29.0,
        31.0,
    ];
}

lazy_static::lazy_static! {
pub static ref FIBONACCI: HashSet<u64> = HashSet::from([
    0,
    1,
    1,
    2,
    3,
    5,
    8,
    13,
    21,
    34,
    55,
    89,
    144,
    233,
    377,
    610,
    987,
    1597,
    2584,
    4181,
    6765,
    10946,
    17711,
    28657,
    46368,
    75025,
    121393,
    196418,
    317811,
    514229,
    832040,
    1346269,
    2178309,
    3524578,
    5702887,
    9227465,
    14930352,
    24157817,
    39088169,
    63245986,
    102334155,
    165580141,
    267914296,
    433494437,
    701408733,
    1134903170,
    1836311903,
    2971215073,
    4807526976,
    7778742049,
    12586269025,
    20365011074,
    32951280099,
    53316291173,
    86267571272,
]);
}

#[derive(Props, PartialEq, Clone, Debug)]
pub struct MatrixItem {
    pub is_shaded: bool,
    pub init_group: Uuid,
    pub value: u8,
    pub group: Uuid,
    pub left: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
}

pub fn matrix_index(row: usize, col: usize) -> usize {
    row * ROWS + col
}

#[derive(Props, PartialEq, Clone)]
pub struct BoxProps {
    pub x: usize,
    pub y: usize,
    pub init_group: Uuid,
    pub shading_mode: Signal<bool>,
}

impl BoxProps {
    pub fn new(x: usize, y: usize, shading_mode: Signal<bool>, init_group: Uuid) -> Self {
        Self {
            x,
            y,
            shading_mode,
            init_group,
        }
    }
}

pub fn does_number_fit_in_row(num: u64, row: &Vec<usize>) -> bool {
    let length_of_n: usize = successors(Some(num), |&num| (num >= 10).then(|| num / 10)).count();
    if length_of_n == 0 || length_of_n == 1 {
        // How could it even be 0? numbers of length 1 are not allowed.
        return false;
    }
    let base: u64 = 10;
    let mut group_to_digit: HashMap<usize, u8> = HashMap::new();
    tracing::debug!("length of n: {}", length_of_n);
    for i in 0..COLS {
        tracing::debug!("i: {}", i);
        if i + length_of_n > COLS {
            break;
        } else if i == 2 || i + length_of_n == COLS - 2 {
            // Can't shade column for index 1 or (COLS - 2).
            continue;
        }
        group_to_digit.clear();
        let mut valid: bool = true;
        let mut n: u64 = num;
        for j in (0..length_of_n).rev() {
            let row_index: usize = j + i;
            let digit: u8 = (n % base) as u8;
            n /= 10;
            assert!(digit < 10);
            let group: usize = row[row_index];
            // First iteration
            if row_index == i + length_of_n - 1 {
                group_to_digit.insert(group, digit);
                continue;
            }
            // All other iterations
            let group_right: usize = row[row_index + 1];
            if group == group_right {
                if &digit != group_to_digit.get(&group_right).unwrap() {
                    valid = false;
                    break;
                }
            } else {
                if &digit == group_to_digit.get(&group_right).unwrap() {
                    valid = false;
                    break;
                }
                group_to_digit.insert(group, digit);
            }
        }
        if valid {
            return true;
        }
    }
    false
}

#[derive(PartialEq, Clone, Debug)]
struct MatrixRow {
    // -1 is not set. -2 is shaded.
    pub row: [i8; COLS],
    pub groups: [usize; COLS],
}

#[derive(Debug)]
enum MatrixRowError {
    RowAlreadyAssigned,
    NumberDoesNotFitPreconditions,
    DigitDoesNotMatchExistingGroup(usize, i8, HashMap<usize, i8>, [usize; COLS], usize),
    CanNotShadeOneOffEdges,
    DigitOthogonallyMatchesAnotherGroup,
}

impl MatrixRow {
    pub fn new(groups: &Vec<usize>) -> Self {
        Self {
            row: [-1; COLS],
            groups: groups.as_slice().try_into().unwrap(),
        }
    }

    pub fn try_insert(
        &mut self,
        num: u64,
        shade_prior_index: bool,
    ) -> Result<bool, MatrixRowError> {
        let n_len: usize = Self::length_of_n(num);
        let mut start_index: usize = match self.latest_unset_index() {
            Some(i) => i,
            None => return Ok(true),
        };
        let mut indexes_set: Vec<usize> = Vec::with_capacity(n_len);
        let mut group_to_digit: HashMap<usize, i8> = HashMap::new();
        if shade_prior_index {
            if start_index == 1 || start_index == self.row.len() - 2 {
                return Err(MatrixRowError::CanNotShadeOneOffEdges);
            }
            self.row[start_index] = -2;
            indexes_set.push(start_index);
            start_index += 1;
        }
        let start_index: usize = start_index;
        if n_len == self.row.len() - 1 || start_index + n_len > self.row.len() || n_len < 2 {
            return Err(MatrixRowError::NumberDoesNotFitPreconditions);
        }
        let num_vec: Vec<i8> = Self::number_to_i8_vec(num);
        for i in start_index..(start_index + n_len) {
            if self.row[i] != -1 {
                // already assigned.
                return Err(MatrixRowError::RowAlreadyAssigned);
            }
        }

        // If everything looks good, go assign those rows.
        for i in start_index..(start_index + n_len) {
            if self.row[i] != -1 {
                for idx in indexes_set {
                    self.row[idx] = -1;
                }
                return Err(MatrixRowError::RowAlreadyAssigned);
            }
            let digit: i8 = num_vec[i - start_index];
            if i != 0
                && self.groups[i] != self.groups[i - 1]
                && self.row[i - 1] >= 0
                && group_to_digit.get(&self.groups[i - 1]).unwrap() == &digit
            {
                return Err(MatrixRowError::DigitOthogonallyMatchesAnotherGroup);
            }

            if group_to_digit.contains_key(&self.groups[i])
                && &digit != group_to_digit.get(&self.groups[i]).unwrap()
            {
                for idx in indexes_set {
                    self.row[idx] = -1;
                }
                return Err(MatrixRowError::DigitDoesNotMatchExistingGroup(
                    i,
                    digit,
                    group_to_digit,
                    self.groups,
                    start_index,
                ));
            }
            if !group_to_digit.contains_key(&self.groups[i]) {
                group_to_digit.insert(self.groups[i], digit);
            }
            self.row[i] = digit;
            indexes_set.push(i);
        }
        for i in 0..self.row.len() {
            if i == self.row.len() - 1 && self.row[i] == -1 {
                // Shade the final spot in the row if it's good!
                self.row[self.row.len() - 1] = -2;
            }
            if self.row[i] == -1 {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn length_of_n(n: u64) -> usize {
        successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count()
    }

    fn latest_unset_index(&self) -> Option<usize> {
        for i in 0..self.row.len() {
            if self.row[i] == -1 {
                return Some(i);
            }
        }
        None
    }

    fn number_to_i8_vec(n: u64) -> Vec<i8> {
        let mut digits: Vec<i8> = Vec::new();
        let mut n: u64 = n;
        while n > 9 {
            digits.push((n % 10) as i8);
            n = n / 10;
        }
        digits.push(n as i8);
        digits.reverse();
        digits
    }
}

fn sets_of_numbers_that_fit_into_row_helper(
    matrix_row: MatrixRow,
    nums: HashSet<u64>,
    all_answers: &mut Vec<MatrixRow>,
) -> bool {
    for num in &nums {
        assert!(matrix_row.latest_unset_index().is_some());
        if matrix_row.latest_unset_index().is_some_and(|x| x == 0) {
            let mut first_index_matrix_row = matrix_row.clone();
            match first_index_matrix_row.try_insert(num.clone(), false) {
                Ok(true) => {
                    all_answers.push(first_index_matrix_row.clone());
                }
                Ok(false) => {
                    let mut next_nums_set: HashSet<u64> = nums.clone();
                    next_nums_set.remove(&num);
                    sets_of_numbers_that_fit_into_row_helper(
                        first_index_matrix_row.clone(),
                        next_nums_set,
                        all_answers,
                    );
                }
                Err(e) => {
                    tracing::debug!("first_index_matrix_row err: {:?}", e);
                }
            }
        }

        // Insert into first available index. Also try adding a shade.
        let mut new_matrix_row = matrix_row.clone();
        match new_matrix_row.try_insert(num.clone(), true) {
            Ok(true) => {
                all_answers.push(new_matrix_row.clone());
            }
            Ok(false) => {
                let mut next_nums_set: HashSet<u64> = nums.clone();
                next_nums_set.remove(&num);
                sets_of_numbers_that_fit_into_row_helper(
                    new_matrix_row.clone(),
                    next_nums_set,
                    all_answers,
                );
            }
            Err(e) => {
                tracing::debug!("new_matrix_row err: {:?}", e);
            }
        }
    }
    false
}

pub fn sets_of_numbers_that_fit_into_row(nums: &Vec<u64>, row: &Vec<usize>) -> Vec<String> {
    // First go through row group numbers and make each same groups but separated into a new group.
    let mut prev_group_num: usize = row[0].clone();
    let mut prev_new_group_num: usize = 0;
    let mut new_row: Vec<usize> = vec![prev_new_group_num];
    for i in 1..row.len() {
        if prev_group_num != row[i] {
            prev_new_group_num += 1;
            prev_group_num = row[i];
        }
        new_row.push(prev_new_group_num)
    }
    let mut possible_answers: Vec<MatrixRow> = Vec::new();
    sets_of_numbers_that_fit_into_row_helper(
        MatrixRow::new(&new_row),
        nums.iter().cloned().collect(),
        &mut possible_answers,
    );
    let mut result: Vec<String> = Vec::new();
    for answer in &possible_answers {
        let mut res: String = String::new();
        for digit in &answer.row {
            if digit == &-2_i8 {
                res += "X";
            } else {
                res += &digit.to_string();
            }
        }
        result.push(res);
    }
    result.sort();
    return result;
}

pub fn get_row_vec_of_init_group_numbers(row_num: usize) -> Vec<usize> {
    let mut groups_in_row: Vec<usize> = Vec::with_capacity(COLS);
    let row_start_index: usize = COLS * row_num;
    for col in 0..COLS {
        groups_in_row.push(GROUP_NUMBERS[row_start_index + col]);
    }
    groups_in_row
}

pub fn sum_digits(x: u64) -> u64 {
    (x % 10)
        + (0..)
            .scan(x, |num, _| {
                *num /= 10;
                Some(*num)
            })
            .take_while(|num| *num > 0)
            .map(|num| num % 10)
            .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_what_row_number_fits() {
        let test_number: u64 = 83_330_993_111;

        let row_0 = get_row_vec_of_init_group_numbers(0);
        let row_1 = get_row_vec_of_init_group_numbers(1);
        let row_2 = get_row_vec_of_init_group_numbers(2);
        let row_3 = get_row_vec_of_init_group_numbers(3);
        let row_4 = get_row_vec_of_init_group_numbers(4);
        let row_5 = get_row_vec_of_init_group_numbers(5);
        let row_6 = get_row_vec_of_init_group_numbers(6);
        let row_7 = get_row_vec_of_init_group_numbers(7);
        let row_8 = get_row_vec_of_init_group_numbers(8);
        let row_9 = get_row_vec_of_init_group_numbers(9);
        let row_10 = get_row_vec_of_init_group_numbers(10);

        println!("Testing number: {}", test_number);
        println!("row_6: {:?}", row_6.clone());
        println!("row_0: {}", does_number_fit_in_row(test_number, &row_0));
        println!("row_1: {}", does_number_fit_in_row(test_number, &row_1));
        println!("row_2: {}", does_number_fit_in_row(test_number, &row_2));
        println!("row_3: {}", does_number_fit_in_row(test_number, &row_3));
        println!("row_4: {}", does_number_fit_in_row(test_number, &row_4));
        println!("row_5: {}", does_number_fit_in_row(test_number, &row_5));
        println!("row_6: {}", does_number_fit_in_row(test_number, &row_6));
        println!("row_7: {}", does_number_fit_in_row(test_number, &row_7));
        println!("row_8: {}", does_number_fit_in_row(test_number, &row_8));
        println!("row_9: {}", does_number_fit_in_row(test_number, &row_9));
        println!("row_10: {}", does_number_fit_in_row(test_number, &row_10));
    }
}
