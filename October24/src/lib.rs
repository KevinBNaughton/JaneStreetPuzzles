use std::{collections::HashSet, usize};

const ROWS: usize = 6;
const COLS: usize = 6;
const ROWS_I8: i8 = 6;
const COLS_I8: i8 = 6;
const BOARD_MAPPING: [[char; COLS]; ROWS] = [
    ['A', 'B', 'B', 'C', 'C', 'C'],
    ['A', 'B', 'B', 'C', 'C', 'C'],
    ['A', 'A', 'B', 'B', 'C', 'C'],
    ['A', 'A', 'B', 'B', 'C', 'C'],
    ['A', 'A', 'A', 'B', 'B', 'C'],
    ['A', 'A', 'A', 'B', 'B', 'C'],
];
const ROW_MAPPING: [char; ROWS] = ['6', '5', '4', '3', '2', '1'];
const COL_MAPPING: [char; COLS] = ['a', 'b', 'c', 'd', 'e', 'f'];
const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

pub struct Board {
    grid: [[u16; COLS]; ROWS],
}

pub struct Game {
    board: Board,
}

impl Board {
    fn new(a: u16, b: u16, c: u16) -> Self {
        let mut grid: [[u16; COLS]; ROWS] = [[0_u16; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                let mapping: char = BOARD_MAPPING[row][col];
                if mapping == 'A' {
                    grid[row][col] = a;
                } else if mapping == 'B' {
                    grid[row][col] = b;
                } else {
                    // mapping == 'C'
                    grid[row][col] = c;
                }
            }
        }

        Self { grid }
    }
    pub fn convert_row_col_to_index(row: i8, col: i8) -> (char, char) {
        return (ROW_MAPPING[row as usize], COL_MAPPING[col as usize])
    }
}

impl Game {
    pub fn new(a: u16, b: u16, c: u16) -> Self {
        Self {
            board: Board::new(a, b, c),
        }
    }
    pub fn evaluate_all_paths(self, start: (i8, i8), target: (i8, i8), max_number_of_paths: usize) -> Vec<Vec<(i8, i8)>> {
        let mut all_paths: Vec<Vec<(i8, i8)>> = Vec::new();
        let mut visited: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];
        visited[usize::try_from(start.0).expect("not usize")]
            [usize::try_from(start.1).expect("not usize")] = true;
        Self::evaluate_path(
            start,
            target,
            &mut Vec::from([start]),
            &mut visited,
            &mut all_paths,
            &max_number_of_paths,
        );
        all_paths
    }

    fn evaluate_path(
        position: (i8, i8),
        target: (i8, i8),
        path: &mut Vec<(i8, i8)>,
        visited: &mut [[bool; COLS]; ROWS],
        all_paths: &mut Vec<Vec<(i8, i8)>>,
        max_number_of_paths: &usize,
    ) {
        // print!("{:?} ", stacker::remaining_stack());
        // println!("evaluate_path: {:?} {:?}", position, target);
        // println!("Visited first check: {:?}", visited);
        if &all_paths.len() >= max_number_of_paths {
            return;
        } else if position == target {
            // println!("Adding path {:?}\nWith visited {:?}", path, visited);
            all_paths.push(path.clone());
            return;
        }
        for knight_move in KNIGHT_MOVES {
            let row: i8 = position.0 + knight_move.0;
            let col: i8 = position.1 + knight_move.1;
            if row < 0 || row >= ROWS_I8 || col < 0 || col >= COLS_I8 {
                continue;
            }
            let row_usize: usize = usize::try_from(row).expect("not usize");
            let col_usize: usize = usize::try_from(col).expect("not usize");
            if visited[row_usize][col_usize] {
                continue;
            }
            visited[row_usize][col_usize] = true;

            let next_position: (i8, i8) = (row, col);
            path.push(next_position);
            Self::evaluate_path(next_position, target, path, visited, all_paths, max_number_of_paths);
            path.pop();
            visited[row_usize][col_usize] = false;
        }
    }
}
