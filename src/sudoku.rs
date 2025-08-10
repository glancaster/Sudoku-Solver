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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_board() {
        let mut sudoku = Sudoku::new();
        let board = [[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9];
        sudoku.set_board(board);
        assert_eq!(sudoku.get_board(), board);
    }
    #[test]
    fn test_set_get_puzzle() {
        let mut sudoku = Sudoku::new();
        let puzzle = [[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9];
        sudoku.set_puzzle(puzzle);
        assert_eq!(sudoku.get_puzzle(), Some(puzzle));
    }
    #[test]
    fn test_set_get_solution() {
        let mut sudoku = Sudoku::new();
        let solution = [[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9];
        sudoku.set_solution(solution);
        assert_eq!(sudoku.get_solution(), Some(solution));
    }
    #[test]
    fn test_set_get_difficulty() {
        let mut sudoku = Sudoku::new();
        let difficulty = Difficulty::Hard;
        sudoku.set_difficulty(Some(difficulty.clone()));
        assert_eq!(sudoku.get_difficulty(), Some(difficulty));
    }
    #[test]
    fn test_clear_board() {
        let mut sudoku = Sudoku::new();
        let board = [[1; 9]; 9];
        let puzzle = [[2; 9]; 9];
        let solution = [[3; 9]; 9];
        sudoku.set_board(board);
        sudoku.set_puzzle(puzzle);
        sudoku.set_solution(solution);
        sudoku.clear_board();
        assert_eq!(sudoku.get_board(), [[0; 9]; 9]);
        assert_eq!(sudoku.get_puzzle(), None);
        assert_eq!(sudoku.get_solution(), None);
    }
}
