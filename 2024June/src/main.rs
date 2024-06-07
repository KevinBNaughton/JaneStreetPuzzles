#![allow(non_snake_case)]

use {
    altered_states_2::{
        matrix_index,
        states::{State, STATES},
        BoxProps, GridProps, MatrixItem, ScoreProps, COLS, ROWS,
    },
    dioxus::prelude::*,
    num_format::{Locale, ToFormattedString},
    std::collections::{HashMap, VecDeque},
    tracing::Level,
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
            r#type: "text",
            flex: 1,
            text_align: "center",
            align_self: "center",
            border_color: "black",
            border_width: "0px",
            border_style: "solid",
            border_left_width: "1px",
            border_top_width: "1px",
            border_right_width: "1px",
            border_bottom_width: "1px",
            overflow: "hidden",
            border_collapse: "separate",
            background_color: "transparent",
            maxlength: 1,
            value: "{matrix.index(m_idx).c}",
            oninput: move |event| {
                if (event.value()) != "" {
                    let mut temp_matrix: Vec<MatrixItem> = (matrix)();
                    temp_matrix[m_idx].c = event.value().clone().parse::<char>().unwrap();
                    matrix.set(temp_matrix);
                }
            },
        }
    }
}

#[allow(non_snake_case)]
fn Grid(props: GridProps) -> Element {
    tracing::info!("Init Grid");
    rsx! {
        div {
            width: "100%",
            height: "50%",
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
                        }
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn Score(props: ScoreProps) -> Element {
    tracing::info!("Init Score");
    let matrix: Signal<Vec<MatrixItem>> = use_context();

    let dir_diffs: Vec<(i8, i8)> = vec![
        (-1, -1), // top left
        (-1, 0),  // top
        (-1, 1),  // top right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // bot left
        (1, 0),   // bot
        (1, 1),   // bot right
    ];
    let cloned_states: Vec<State> = props.states.clone();
    let memo_states: Memo<(HashMap<String, (u32, String, bool)>, u32, String)> =
        use_memo(move || {
            let mut complete_string = String::new();
            for matrix_item in &(matrix)() {
                complete_string += matrix_item.c.to_string().as_str();
            }
            let mut states_map = HashMap::<String, (u32, String, bool)>::new();
            for row in 0..ROWS {
                for col in 0..COLS {
                    let item: &MatrixItem = &(matrix)()[matrix_index(row, col)];
                    for state in &cloned_states {
                        let state_as_chars: Vec<char> = state.name.chars().collect();
                        let mut queue: VecDeque<(usize, usize, String, bool)> = VecDeque::new();
                        let mut queue_2: VecDeque<(usize, usize, String, bool)> = VecDeque::new();
                        let states_len: usize = state.name.len();
                        if item.c == state_as_chars[0] {
                            queue.push_back((row, col, item.c.to_string().clone(), false));
                        } else {
                            queue.push_back((row, col, '?'.to_string(), true));
                        }
                        while !queue.is_empty() {
                            if queue.front().unwrap().2.len() >= states_len {
                                break;
                            }
                            let (n_row, n_col, s, wildcard_used) = queue.pop_front().unwrap();
                            for (row_diff, col_diff) in &dir_diffs {
                                let nn_row: i8 = n_row as i8 + row_diff;
                                let nn_col: i8 = n_col as i8 + col_diff;
                                if nn_row < 0
                                    || nn_row >= ROWS as i8
                                    || nn_col < 0
                                    || nn_col >= COLS as i8
                                {
                                    continue;
                                }
                                let next: &MatrixItem =
                                    &(matrix)()[matrix_index(nn_row as usize, nn_col as usize)];
                                if wildcard_used && next.c != state_as_chars[s.len()] {
                                    // nothing
                                } else if next.c != state_as_chars[s.len()] {
                                    queue_2.push_back((
                                        nn_row as usize,
                                        nn_col as usize,
                                        s.clone() + "?",
                                        true,
                                    ));
                                } else {
                                    queue_2.push_back((
                                        nn_row as usize,
                                        nn_col as usize,
                                        s.clone() + next.c.to_string().as_str(),
                                        wildcard_used,
                                    ));
                                }
                            }
                            if queue.is_empty() {
                                queue = queue_2.clone();
                                queue_2 = VecDeque::new();
                            }
                        }
                        if !queue.is_empty() {
                            states_map.insert(
                                state.name.clone(),
                                (state.population, queue.pop_front().unwrap().2, true),
                            );
                        }
                    }
                }
            }
            let mut total_pop: u32 = 0;
            for (pop, _, _) in states_map.values() {
                total_pop += pop;
            }
            (states_map, total_pop, complete_string)
        });
    rsx! {
        div {
            height: "50px",
            width: "100%",
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            position: "relative",
            div {
                width: "50%",
                height: "49px",
                text_align: "center",
                vertical_align: "middle",
                line_height: "49px",
                justify_content: "center",
                "Total population: {(memo_states()).1.to_formatted_string(&Locale::en)}"
            }
            div {
                width: "50%",
                height: "49px",
                text_align: "center",
                vertical_align: "middle",
                line_height: "49px",
                justify_content: "center",
                "Answer: {(memo_states()).2}"
            }
        }
        div {
            display: "flex",
            overflow: "auto",
            align_content: "flex-start",
            flex_direction: "column",
            flex_wrap: "wrap",
            height: "701px",
            for i in 0..props.states.len() {
                div {
                    height: "25px",
                    width: "49%",
                    border_width: "1px",
                    border_style: "solid",
                    flex_direction: "row",
                    flex_wrap: "nowrap",
                    position: "relative",
                    display: "flex",
                    justify_content: "space-between",
                    background: match (memo_states()).0.get(&props.states[i].name) {
                        Some(p) => {
                            if p.2 {
                                "#AFE1AF"
                            } else {
                                ""
                            }
                        },
                        None => "",
                    },
                    div {
                        width: "30%",
                        "{props.states[i].name}"
                    }
                    div {
                        width: "25%",
                        "Pop: {props.states[i].population.to_formatted_string(&Locale::en)}"
                    }

                    div {
                        width: "35%",
                        match (memo_states()).0.get(&props.states[i].name) {
                            Some(part_val_and_complete) => part_val_and_complete.1.clone(),
                            None => "".to_string(),
                        }
                    }
                    div {
                        width: "10%",
                        match (memo_states()).0.get(&props.states[i].name) {
                            Some(part_val_and_complete) => part_val_and_complete.2.to_string(),
                            None => false.to_string(),
                        }
                    }
                }
            }
        }
    }
}

fn App() -> Element {
    let mut states: Vec<State> = STATES.to_vec();
    states.sort_by(|a, b| b.population.cmp(&a.population));
    let states: Vec<State> = states;

    use_context_provider(|| {
        let mut vec: Vec<MatrixItem> = Vec::new();
        for _ in 0..(ROWS * COLS) {
            vec.push(MatrixItem { c: 'S' });
        }
        Signal::new(vec)
    });
    let mut box_props: Vec<BoxProps> = Vec::new();
    for row in 0..ROWS {
        for col in 0..COLS {
            box_props.push(BoxProps::new(row, col));
        }
    }

    let grid_props: GridProps = GridProps { box_props };

    rsx! {
        div{
            width: "100%",
            height: "100%",
            div {
                flex_direction: "column",
                flex_wrap: "nowrap",
                position: "relative",
                display: "flex",
                justify_content: "space-between",
                Grid {
                    box_props: grid_props.box_props,
                }
                Score { states }
            }
        }
    }
}
