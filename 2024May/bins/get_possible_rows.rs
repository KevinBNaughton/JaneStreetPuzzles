use {
    number_cross_4::{
        does_number_fit_in_row, palindrome::Palindrome, primes_27293::PRIMES,
        sets_of_numbers_that_fit_into_row, sum_digits, FIBONACCI, LOW_PRIMES,
    },
    std::{
        fs::File,
        io::{BufWriter, Write},
    },
    tracing::Level,
};

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Row 0 and 5.
    let mut squares_row_0: Vec<u64> = Vec::with_capacity(100_000);
    let mut squares_row_5: Vec<u64> = Vec::with_capacity(100_000);
    // Row 1.
    let mut palindromes_plus_1_row_1: Vec<u64> = Vec::with_capacity(100_000);
    // Row 3.
    let mut sum_digits_7_row_3: Vec<u64> = Vec::with_capacity(100_000);
    // Row 4.
    let mut fibs_row_4: Vec<u64> = Vec::with_capacity(1_000);
    // Row 6.
    let mut mult_of_37_row_6: Vec<u64> = Vec::with_capacity(100_000);
    // Row 7.
    let mut palindrome_mult_23_row_7: Vec<u64> = Vec::with_capacity(100_000);
    // Row 8 didn't need.
    // let mut prod_digits_end_1_row_8: Vec<u64> = Vec::with_capacity(100_000);
    // Row 9.
    let mut mult_of_88_row_9: Vec<u64> = Vec::with_capacity(100_000);
    // Row 10.
    let mut palindromes_minus_1_row_10: Vec<u64> = Vec::with_capacity(100_000);

    // Get groups numbers for the rows.
    let row_0: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(0);
    let row_1: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(1);
    let row_2: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(2);
    let row_3: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(3);
    let row_4: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(4);
    let row_5: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(5);
    let row_6: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(6);
    let row_7: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(7);
    // let row_8: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(8);
    let row_9: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(9);
    let row_10: Vec<usize> = number_cross_4::get_row_vec_of_init_group_numbers(10);

    let squares_limit: u64 = (100_000_000_000_u64 as f64).sqrt() as u64;

    for n in 1..100_000_000_000_u64 {
        // Row 0 and 5.
        if n <= squares_limit {
            let squared: u64 = n * n;
            if does_number_fit_in_row(squared, &row_0) {
                squares_row_0.push(n);
            }
            if does_number_fit_in_row(squared, &row_5) {
                squares_row_5.push(n);
            }
        }
        // Row 1.
        if Palindrome::new(n - 1).is_some() {
            if does_number_fit_in_row(n, &row_1) {
                palindromes_plus_1_row_1.push(n);
            }
        }
        // Row 2 below.
        // Row 3.
        if sum_digits(n) == 7 {
            if does_number_fit_in_row(n, &row_3) {
                sum_digits_7_row_3.push(n);
            }
        }
        // Row 4 below.
        // Row 5 above.
        // Row 6.
        if n % 37 == 0 {
            if does_number_fit_in_row(n, &row_6) {
                mult_of_37_row_6.push(n);
            }
        }
        // Row 7.
        if n % 23 == 0 && Palindrome::new(n).is_some() {
            if does_number_fit_in_row(n, &row_7) {
                palindrome_mult_23_row_7.push(n);
            }
        }
        // Row 8. Not needed.
        // Row 9.
        if n % 88 == 0 {
            if does_number_fit_in_row(n, &row_9) {
                mult_of_88_row_9.push(n);
            }
        }
        // Row 10.
        if Palindrome::new(n + 1).is_some() {
            if does_number_fit_in_row(n, &row_10) {
                palindromes_minus_1_row_10.push(n);
            }
        }
    }

    // Row 2.
    let mut prime_to_primes_row_2: Vec<u64> = Vec::with_capacity(100_000);
    for prime_higher in PRIMES.iter() {
        for prime_low in LOW_PRIMES.iter() {
            let prime_to_a_prime: f64 = (prime_higher.clone() as f64).powf(prime_low.clone());
            if prime_to_a_prime > 99_999_999_999.0 {
                break;
            }
            if does_number_fit_in_row(prime_to_a_prime as u64, &row_2) {
                prime_to_primes_row_2.push(prime_to_a_prime as u64);
            }
        }
    }

    // Row 4.
    for fib_num in FIBONACCI.iter() {
        if does_number_fit_in_row(fib_num.clone(), &row_4) {
            fibs_row_4.push(fib_num.clone());
        }
    }

    // Sort them.
    squares_row_0.sort();
    palindromes_plus_1_row_1.sort();
    prime_to_primes_row_2.sort();
    sum_digits_7_row_3.sort();
    fibs_row_4.sort();
    squares_row_5.sort();
    mult_of_37_row_6.sort();
    palindrome_mult_23_row_7.sort();
    // prod_digits_end_1_row_8.sort();
    mult_of_88_row_9.sort();
    palindromes_minus_1_row_10.sort();

    // Get real possible combos.
    let possible_row_0: Vec<String> = sets_of_numbers_that_fit_into_row(&squares_row_0, &row_0);
    let possible_row_1: Vec<String> =
        sets_of_numbers_that_fit_into_row(&palindromes_plus_1_row_1, &row_1);
    let possible_row_2: Vec<String> =
        sets_of_numbers_that_fit_into_row(&prime_to_primes_row_2, &row_2);
    let possible_row_3: Vec<String> =
        sets_of_numbers_that_fit_into_row(&sum_digits_7_row_3, &row_3);
    let possible_row_4: Vec<String> = sets_of_numbers_that_fit_into_row(&fibs_row_4, &row_4);
    let possible_row_5: Vec<String> = sets_of_numbers_that_fit_into_row(&squares_row_5, &row_5);
    let possible_row_6: Vec<String> = sets_of_numbers_that_fit_into_row(&mult_of_37_row_6, &row_6);
    let possible_row_7: Vec<String> =
        sets_of_numbers_that_fit_into_row(&palindrome_mult_23_row_7, &row_7);
    // let possible_row_8: Vec<String> =
    //     sets_of_numbers_that_fit_into_row(&prod_digits_end_1_row_8, &row_8);
    let possible_row_9: Vec<String> = sets_of_numbers_that_fit_into_row(&mult_of_88_row_9, &row_9);
    let possible_row_10: Vec<String> =
        sets_of_numbers_that_fit_into_row(&palindromes_minus_1_row_10, &row_10);

    // Save them to read them.
    save_to_file(possible_row_0, "rows/row_0.json");
    save_to_file(possible_row_1, "rows/row_1.json");
    save_to_file(possible_row_2, "rows/row_2.json");
    save_to_file(possible_row_3, "rows/row_3.json");
    save_to_file(possible_row_4, "rows/row_4.json");
    save_to_file(possible_row_5, "rows/row_5.json");
    save_to_file(possible_row_6, "rows/row_6.json");
    save_to_file(possible_row_7, "rows/row_7.json");
    // save_to_file(possible_row_8, "rows/row_8.json");
    save_to_file(possible_row_9, "rows/row_9.json");
    save_to_file(possible_row_10, "rows/row_10.json");
}

fn save_to_file(data: Vec<String>, file_name: &str) {
    let file = File::create(file_name).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &data).unwrap();
    writer.flush().unwrap();
}
