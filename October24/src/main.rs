use knightmoves6::{Board, Game};
use std::fs;

fn convert_index_to_str_paths(paths: &Vec<Vec<(i8, i8)>>) -> Vec<String> {
    let mut paths_as_str: Vec<String> = Vec::with_capacity(paths.len());
    for path in paths {
        let mut path_str: String = String::new();
        for index in path {
            let chars = Board::convert_row_col_to_index(index.0, index.1);
            path_str.push(chars.0);
            path_str.push(chars.1);
            path_str.push(',');
        }
        path_str.pop();
        paths_as_str.push(path_str);
        // println!("Path: {:?}", path_str);
    }
    paths_as_str
}

fn main() {
    println!("{:?}", stacker::remaining_stack());
    stacker::grow(10 * 1024 * 1024 * 1024 * 1024, || {
        // guaranteed to have at least 10T of stack
        let game: Game = Game::new(0, 0, 0);
        // a1 to f6
        let paths_1 = game.evaluate_all_paths(
            (5, 0),
            (0, 5),
            100_000_000
        );
        let paths_str_1: Vec<String> = convert_index_to_str_paths(&paths_1);
        let as_string = paths_str_1.join("\n");
        // println!("as_string {:?}", as_string);
        fs::write("paths_1.txt", as_string).expect("Unable to write paths_1.txt file.");
    });
}
