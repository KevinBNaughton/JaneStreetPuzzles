use dioxus::prelude::*;
use states::State;
pub mod states;

pub const ROWS: usize = 5;
pub const COLS: usize = 5;

#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    pub box_props: Vec<BoxProps>,
}

#[derive(Props, PartialEq, Clone, Debug)]
pub struct MatrixItem {
    pub c: char,
}

pub fn matrix_index(row: usize, col: usize) -> usize {
    row * ROWS + col
}

#[derive(Props, PartialEq, Clone)]
pub struct BoxProps {
    pub x: usize,
    pub y: usize,
}

impl BoxProps {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ScoreProps {
    pub states: Vec<State>,
}
