#![allow(non_snake_case)]

use {
    dioxus::prelude::*,
    number_cross_4::{
        matrix_index, palindrome::Palindrome, primes_27293::PRIMES, BoxProps, MatrixItem, COLS,
        FIBONACCI, GROUP_NUMBERS, LOW_PRIMES, ROWS,
    },
    std::collections::{HashMap, VecDeque},
    tracing::Level,
    uuid::Uuid,
};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[allow(non_snake_case)]
fn GridBox(props: BoxProps) -> Element {
    let mut matrix: Signal<Vec<MatrixItem>> = use_context();
    let m_idx: usize = matrix_index(props.x, props.y);
    rsx! {
        input {
            line_height: "30px",
            height: "30px",
            width: "30px",
            r#type: "number",
            flex: 1,
            text_align: "center",
            align_self: "center",
            border_color: "black",
            border_width: "0px",
            border_style: "solid",
            border_left_width: if matrix.index(m_idx).left {"1px"} else {"0px"},
            border_top_width: if matrix.index(m_idx).top {"1px"} else {"0px"},
            border_right_width: if matrix.index(m_idx).right {"1px"} else {"0px"},
            border_bottom_width: if matrix.index(m_idx).bottom {"1px"} else {"0px"},
            overflow: "hidden",
            border_collapse: "separate",
            background_color: if matrix.index(m_idx).is_shaded {"black"} else {"transparent"},
            maxlength: 1,
            value: "{matrix.index(m_idx).value}",
            oninput: move |event| {
                if (event.value()) != "" {
                    let mut temp_matrix: Vec<MatrixItem> = (matrix)();
                    let mut to_update: Vec<usize> = Vec::new();
                    let cur_group: Uuid = temp_matrix[m_idx].group;
                    for i in 0..temp_matrix.len() {
                        // if GROUP_NUMBERS[i] == cur_group {
                        if temp_matrix[i].group == cur_group {
                            to_update.push(i);
                        }
                    }
                    let new_value = event.value().clone().parse::<u8>().unwrap();
                    for index in to_update {
                        temp_matrix[index].value = new_value;
                    }
                    matrix.set(temp_matrix);
                }
            },
            onclick: move |event| {
                tracing::info!("Clicked! Event: {event:?}");
                if (props.shading_mode)() {
                    let mut temp_matrix: Vec<MatrixItem> = (matrix)();
                    tracing::info!("Box {}-{} is_shaded: {}", props.x, props.y, !temp_matrix[m_idx].is_shaded);
                    tracing::debug!("Shaded. Initial temp_matrix: {:#?}", temp_matrix);
                    for i in 0..temp_matrix.len() {
                        tracing::info!("PRE {}-{}: {:?}", i / ROWS, i % ROWS, &temp_matrix[i]);
                    }
                    temp_matrix[m_idx].is_shaded = !temp_matrix[m_idx].is_shaded;

                    let mut indexes_that_matter: Vec<usize> = Vec::new();
                    let mut visited: Vec<bool> = Vec::new();
                    for i in 0..temp_matrix.len() {
                        if temp_matrix[i].init_group == props.init_group && !temp_matrix[i].is_shaded {
                            indexes_that_matter.push(i);
                            visited.push(false);
                        } else {
                            visited.push(true);
                        }
                    }
                    tracing::info!("indexes_that_matter created: {:?}", indexes_that_matter);
                    let mut initial_queue: VecDeque<usize> = VecDeque::with_capacity(4);
                    // left border
                    if !temp_matrix[m_idx].left && !temp_matrix[m_idx - 1].is_shaded {
                        initial_queue.push_back(m_idx - 1);
                    }
                    // top border
                    if !temp_matrix[m_idx].top && !temp_matrix[m_idx - COLS].is_shaded {
                        initial_queue.push_back(m_idx - COLS);
                    }
                    // right border
                    if !temp_matrix[m_idx].right && !temp_matrix[m_idx + 1].is_shaded {
                        initial_queue.push_back(m_idx + 1);
                    }
                    // bottom border
                    if !temp_matrix[m_idx].bottom  && !temp_matrix[m_idx + COLS].is_shaded {
                        initial_queue.push_back(m_idx + COLS);
                    }
                    while !initial_queue.is_empty() {
                        let start_index: usize = initial_queue.pop_front().unwrap();
                        if visited[start_index] {
                            continue;
                        }
                        let mut q: VecDeque<usize> = VecDeque::new();
                        q.push_back(start_index);
                        let new_group_hash: Uuid = Uuid::new_v4();
                        while !q.is_empty() {
                            let i: usize = q.pop_front().unwrap();
                            if visited[i] {
                                continue;
                            }
                            temp_matrix[i].group = new_group_hash;
                            visited[i] = true;
                            // left border
                            if !temp_matrix[i].left && !temp_matrix[i - 1].is_shaded {
                                q.push_back(i - 1);
                            }
                            // top border
                            if !temp_matrix[i].top && !temp_matrix[i - COLS].is_shaded {
                                q.push_back(i - COLS);
                            }
                            // right border
                            if !temp_matrix[i].right && !temp_matrix[i + 1].is_shaded {
                                q.push_back(i + 1);
                            }
                            // bottom border
                            if !temp_matrix[i].bottom  && !temp_matrix[i + COLS].is_shaded {
                                q.push_back(i + COLS);
                            }
                        }
                        if initial_queue.is_empty() {
                            for visit_id in 0..visited.len() {
                                if !visited[visit_id] {
                                    initial_queue.push_back(visit_id);
                                }
                            }
                        }
                    }
                    for i in 0..temp_matrix.len() {
                        tracing::info!("END {}-{}: {:?}", i / ROWS, i % ROWS, &temp_matrix[i]);
                    }
                    // End off by setting updated matrix.
                    matrix.set(temp_matrix);
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct GridProps {
    box_props: Vec<BoxProps>,
}

#[allow(non_snake_case)]
fn Grid(props: GridProps) -> Element {
    tracing::info!("Init Grid");
    rsx! {
        div {
            width: "74%",
            height: "100%",
            flex_direction: "column",
            flex_wrap: "nowrap",
            position: "relative",
            display: "flex",
            for x in 0..ROWS {
                div {
                    height: "30px",
                    flex: 1,
                    display: "flex",
                    flex_wrap: "nowrap",
                    flex_direction: "row",
                    for y in 0..COLS {
                        GridBox {
                            key: "{x}-{y}",
                            x: props.box_props[matrix_index(x, y)].x,
                            y: props.box_props[matrix_index(x, y)].y,
                            init_group: props.box_props[matrix_index(x, y)].init_group,
                            // matrix: props.box_props[matrix_index(x, y)].matrix,
                            shading_mode: props.box_props[matrix_index(x, y)].shading_mode,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ScoreProps {
    shading_mode: Signal<bool>,
}

#[allow(non_snake_case)]
fn Score(mut props: ScoreProps) -> Element {
    tracing::info!("Init Score");
    let matrix: Signal<Vec<MatrixItem>> = use_context();

    fn isqrt(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut s = (n as f64).sqrt() as u64;
        s = (s + n / s) >> 1;
        if s * s > n {
            s - 1
        } else {
            s
        }
    }

    fn perfect_sqrt(n: u64) -> Option<u64> {
        match n & 0xf {
            0 | 1 | 4 | 9 => {
                let t: u64 = isqrt(n);
                if t * t == n {
                    Some(t)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_vals_in_row(matrix: Signal<Vec<MatrixItem>>, row: usize) -> Vec<u64> {
        let mut vals: Vec<u64> = Vec::new();
        let mut temp_val: String = "".to_string();
        let row_start: usize = row * ROWS;
        for i in row_start..(row_start + COLS) {
            let matrix_item: &MatrixItem = &(matrix)()[i];
            if matrix_item.is_shaded {
                if temp_val.is_empty() {
                    continue;
                }
                vals.push(temp_val.parse::<u64>().unwrap());
                temp_val = "".to_string();
                continue;
            }
            temp_val += matrix_item.value.to_string().as_str();
        }
        if !temp_val.is_empty() {
            vals.push(temp_val.parse::<u64>().unwrap());
        }
        vals
    }

    // square
    let memo_0: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 0);
        for val in vals {
            if perfect_sqrt(val).is_none() {
                return false;
            }
        }
        true
    });

    // one more than a palindrome
    let memo_1: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 1);
        for val in vals {
            if Palindrome::new(val - 1).is_none() {
                return false;
            }
        }
        true
    });

    // prime raised to a prime power
    let memo_2: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 2);
        for val in vals {
            tracing::info!("for loop val: {}", val);
            let mut is_prime_of_prime: bool = false;
            // f64::powf(num, 1.0 / nth)
            for n in LOW_PRIMES.clone() {
                let root = f64::powf(val as f64, 1.0 / n);
                if (root as u64) == 0_u64 {
                    tracing::info!(
                        "Returning false, root is 0. for val: {} for n {} and root {}",
                        val,
                        n,
                        root
                    );
                    return false;
                }
                if root <= ((root.round() as u64) as f64) - 0.00001
                    || root >= ((root.round() as u64) as f64) + 0.00001
                {
                    tracing::info!(
                        "yeet Continuing for val: {} for n {} and root {}",
                        val,
                        n,
                        root
                    );
                    continue;
                }
                if PRIMES.contains(&(root.round() as u64)) {
                    is_prime_of_prime = true;
                    tracing::info!(
                        "TRUE! val: {} for n {} and root.round() {}",
                        val,
                        n,
                        root.round()
                    );
                    break;
                }
            }
            if !is_prime_of_prime {
                tracing::info!("val {} is not prime ^ prime", val);
                return false;
            }
        }
        true
    });

    // sum of digits is 7
    let memo_3: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 3);
        for val in vals {
            let mut total: u64 = 0;
            for byte in val.to_string().as_bytes() {
                let digit_opt: Option<u64> = match std::str::from_utf8(&[byte.clone()]) {
                    Ok(digit_as_str) => match digit_as_str.parse::<u64>() {
                        Ok(digit) => Some(digit),
                        Err(_) => None,
                    },
                    Err(_) => None,
                };
                if digit_opt.is_none() {
                    return false;
                }
                total += digit_opt.unwrap();
            }
            if total != 7 {
                return false;
            }
        }
        true
    });

    // fibonacci
    let memo_4: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 4);
        for val in vals {
            if !FIBONACCI.contains(&val) {
                return false;
            }
        }
        true
    });

    // square
    let memo_5: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 5);
        for val in vals {
            if perfect_sqrt(val).is_none() {
                return false;
            }
        }
        true
    });
    // multiple of 37
    let memo_6: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 6);
        for val in vals {
            if val % 37 != 0 {
                return false;
            }
        }
        true
    });
    // palindrome multiple of 23
    let memo_7: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 7);
        for val in vals {
            if val % 23 != 0 || Palindrome::new(val).is_none() {
                return false;
            }
        }
        true
    });
    // product of digits end in 1
    let memo_8: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 8);
        for val in vals {
            let mut product: u64 = 1;
            for byte in val.to_string().as_bytes() {
                let digit_opt: Option<u64> = match std::str::from_utf8(&[byte.clone()]) {
                    Ok(digit_as_str) => match digit_as_str.parse::<u64>() {
                        Ok(digit) => Some(digit),
                        Err(_) => None,
                    },
                    Err(_) => None,
                };
                if digit_opt.is_none() {
                    return false;
                }
                tracing::info!("Digit: {}", digit_opt.unwrap());
                product *= digit_opt.unwrap();
            }
            tracing::info!("product is {}", product);
            if product % 10 == 1 {
                continue;
            } else {
                return false;
            }
        }
        true
    });
    // multiple of 88
    let memo_9: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 9);
        for val in vals {
            if val % 88 != 0 {
                return false;
            }
        }
        true
    });
    // one less than a palindrome
    let memo_10: Memo<bool> = use_memo(move || {
        let vals: Vec<u64> = get_vals_in_row(matrix, 10);
        for val in vals {
            if Palindrome::new(val + 1).is_none() {
                return false;
            }
        }
        true
    });

    // There are 4 booleans. Each represent an extra check. `true` is pass, `false` is fail.
    // Each number must be at least 2 digits long.
    // No shaded box may share an orthogonal edge.
    // No leading zeros for a number. So no zeros.
    // No two groups may share the same value when orthogonal.
    let memo_extra_checks: Memo<(bool, bool, bool, bool)> = use_memo(move || {
        let (
            mut two_or_more_digits_check,
            mut shading_check,
            mut leading_zero_check,
            mut groups_check,
        ) = (true, true, true, true);
        for row in 0..ROWS {
            let mut temp_val: String = "".to_string();
            for col in 0..COLS {
                let item: &MatrixItem = &(matrix)()[matrix_index(row, col)];
                // Check right MatrixItem.
                if col != COLS - 1 {
                    let right_item = &(matrix)()[matrix_index(row, col + 1)];
                    // Check shading.
                    if item.is_shaded && right_item.is_shaded {
                        shading_check = false;
                    }
                    // Check group and values.
                    if !item.is_shaded
                        && !right_item.is_shaded
                        && item.group != right_item.group
                        && item.value == right_item.value
                    {
                        groups_check = false;
                    }
                };
                // Check bottom MatrixItem.
                if row != ROWS - 1 {
                    let bot_item = &(matrix)()[matrix_index(row + 1, col)];
                    // Check shading.
                    if item.is_shaded && bot_item.is_shaded {
                        shading_check = false;
                    }
                    // Check group and values.
                    if !item.is_shaded
                        && !bot_item.is_shaded
                        && item.group != bot_item.group
                        && item.value == bot_item.value
                    {
                        groups_check = false;
                    }
                };
                // Check leading zero.
                if !item.is_shaded && temp_val.len() == 0 && item.value == 0 {
                    leading_zero_check = false;
                }
                // Check at least 2 values.
                if item.is_shaded && temp_val.len() == 1 {
                    two_or_more_digits_check = false;
                }

                // Then go update the value to check for at least 2 later.
                if item.is_shaded {
                    temp_val.clear();
                } else {
                    temp_val += item.value.to_string().as_str();
                }
            }
            // Check last value too.
            if temp_val.len() == 1 {
                two_or_more_digits_check = false;
            }
            temp_val.clear();
            // Return early if all false.
            if !two_or_more_digits_check && !shading_check && !leading_zero_check && !groups_check {
                return (false, false, false, false);
            }
        }
        (
            two_or_more_digits_check,
            shading_check,
            leading_zero_check,
            groups_check,
        )
    });

    rsx! {
        div {
            width: "25%",
            height: "100%",
            flex_direction: "column",
            div {
                height: "34px",
                background: if memo_0() {"#AFE1AF"} else {"#FF5733"},
                "square: {memo_0}"
            }
            div {
                height: "34px",
                background: if memo_1() {"#AFE1AF"} else {"#FF5733"},
                "palindrome+1: {memo_1}"
            }
            div {
                height: "33px",
                background: if memo_2() {"#AFE1AF"} else {"#FF5733"},
                "prime ^ prime: {memo_2}"
            }
            div {
                height: "34px",
                background: if memo_3() {"#AFE1AF"} else {"#FF5733"},
                "sum digits is 7: {memo_3}"
            }
            div {
                height: "34px",
                background: if memo_4() {"#AFE1AF"} else {"#FF5733"},
               "fibonacci: {memo_4}"
            }
            div {
                height: "34px",
                background: if memo_5() {"#AFE1AF"} else {"#FF5733"},
                "square: {memo_5}"
            }
            div {
                height: "34px",
                background: if memo_6() {"#AFE1AF"} else {"#FF5733"},
                "multiple of 37: {memo_6}"
            }
            div {
                height: "33px",
                background: if memo_7() {"#AFE1AF"} else {"#FF5733"},
                "palindrome multiple of 23: {memo_7}"
            }
            div {
                height: "34px",
                background: if memo_8() {"#AFE1AF"} else {"#FF5733"},
                "product digits end in 1: {memo_8}"
            }
            div {
                height: "34px",
                background: if memo_9() {"#AFE1AF"} else {"#FF5733"},
                "multiple of 88: {memo_9}"
            }
            div {
                height: "34px",
                background: if memo_10() {"#AFE1AF"} else {"#FF5733"},
                "palindrome-1: {memo_10}"
            }
            div {
                height: "34px",
                display: "flex",
                flex_direction: "row",
                flex_wrap: "nowrap",
                justify_content: "space-between",
                div {
                    background: if memo_extra_checks().0 {"#AFE1AF"} else {"#FF5733"},
                    "2+digits: {memo_extra_checks().0}"
                }
                div {
                    background: if memo_extra_checks().1 {"#AFE1AF"} else {"#FF5733"},
                    "shading: {memo_extra_checks().1}"
                }
                div {
                    background: if memo_extra_checks().2 {"#AFE1AF"} else {"#FF5733"},
                    "zeroes: {memo_extra_checks().2}"
                }
                div {
                    background: if memo_extra_checks().3 {"#AFE1AF"} else {"#FF5733"},
                    "groups: {memo_extra_checks().3}"
                }
            }
            button {
                height: "34px",
                onclick: move |_| props.shading_mode.set(!(props.shading_mode)()), "Shading Mode: {props.shading_mode}"
            }
        }
    }
}

#[derive(Default, Clone)]
struct InitialBorders {
    left: bool,
    top: bool,
    right: bool,
    bottom: bool,
}

fn App() -> Element {
    let mut initial_borders: Vec<InitialBorders> = Vec::new();
    for i in 0..(ROWS * COLS) {
        let x = i / ROWS;
        let y = i % ROWS;
        let mut init_border: InitialBorders = InitialBorders::default();
        // left border.
        if y == 0 || GROUP_NUMBERS[i] != GROUP_NUMBERS[i - 1] {
            init_border.left = true;
        }
        // top border.
        if x == 0 || GROUP_NUMBERS[i] != GROUP_NUMBERS[i - ROWS] {
            init_border.top = true;
        }
        // right border.
        if y == COLS - 1 || GROUP_NUMBERS[i] != GROUP_NUMBERS[i + 1] {
            init_border.right = true;
        }
        // bottom border.
        if x == ROWS - 1 || GROUP_NUMBERS[i] != GROUP_NUMBERS[i + ROWS] {
            init_border.bottom = true;
        }
        initial_borders.push(init_border);
    }
    let initial_borders: Vec<InitialBorders> = initial_borders;

    use_context_provider(|| {
        let initial_borders_cloned: Vec<InitialBorders> = initial_borders.clone();
        let mut group_num_to_hash: HashMap<usize, Uuid> = HashMap::new();
        for group in GROUP_NUMBERS {
            if !group_num_to_hash.contains_key(group) {
                group_num_to_hash.insert(group.clone(), Uuid::new_v8([group.clone() as u8; 16]));
            }
        }
        let mut vec: Vec<MatrixItem> = Vec::new();
        for i in 0..(ROWS * COLS) {
            let group_hash: Uuid = group_num_to_hash.get(&GROUP_NUMBERS[i]).unwrap().clone();
            vec.push(MatrixItem {
                is_shaded: false,
                init_group: group_hash,
                value: (GROUP_NUMBERS[i] % 10) as u8,
                group: group_hash,
                left: initial_borders_cloned[i].left,
                top: initial_borders_cloned[i].top,
                right: initial_borders_cloned[i].right,
                bottom: initial_borders_cloned[i].bottom,
            });
        }
        Signal::new(vec)
    });
    let shading_mode: Signal<bool> = use_signal(|| false);

    let mut box_props: Vec<BoxProps> = Vec::new();
    for row in 0..ROWS {
        for col in 0..COLS {
            box_props.push(BoxProps::new(
                row,
                col,
                shading_mode,
                Uuid::new_v8([GROUP_NUMBERS[row * ROWS + col].clone() as u8; 16]),
            ));
        }
    }

    let grid_props: GridProps = GridProps { box_props };

    rsx! {
        div{
            width: "100%",
            height: "100%",
            div {
                flex_direction: "row",
                flex_wrap: "nowrap",
                position: "relative",
                display: "flex",
                Grid {
                    box_props: grid_props.box_props,
                }
                // Put buttons and stuff here.
                Score { shading_mode }
            }
        }
    }
}
