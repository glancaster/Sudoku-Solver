use std::sync::{Arc, Mutex};

use crate::solver;

#[derive(Clone)]
pub struct Sudoku {
    board: [[usize; 9]; 9],
    puzzle: Option<[[usize; 9]; 9]>,
    solution: Option<[[usize; 9]; 9]>,
    difficulty: Option<Difficulty>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            board: [[0; 9]; 9],
            puzzle: None,
            solution: None,
            difficulty: None,
        }
    }
    pub fn get_board(&self) -> [[usize; 9]; 9] {
        self.board.clone()
    }
    pub fn set_board(&mut self, board: [[usize; 9]; 9]) {
        self.board = board;
    }
    pub fn get_puzzle(&self) -> Option<[[usize; 9]; 9]> {
        self.puzzle.clone()
    }
    pub fn set_puzzle(&mut self, puzzle: [[usize; 9]; 9]) {
        self.puzzle = Some(puzzle);
    }
    pub fn get_solution(&self) -> Option<[[usize; 9]; 9]> {
        self.solution.clone()
    }
    pub fn set_solution(&mut self, solution: [[usize; 9]; 9]) {
        self.solution = Some(solution);
    }
    pub fn clear_board(&mut self) {
        self.board = [[0; 9]; 9];
        self.puzzle = None;
        self.solution = None;
        self.difficulty = None;
    }
    pub fn update_cell(&mut self, row: usize, col: usize, num: usize) {
        self.board[row][col] = num;
    }
    pub fn get_difficulty(&self) -> Option<Difficulty> {
        self.difficulty.clone()
    }
    pub fn set_difficulty(&mut self, diff: Option<Difficulty>) {
        self.difficulty = diff;
    }
}
