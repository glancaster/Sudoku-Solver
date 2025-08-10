#[derive(Clone)]
pub struct Sudoku {
    /// Representation of the sudoku board
    /// Option not needed since we want to
    /// display whatever state it is at.
    board: [[usize; 9]; 9],
    /// Puzzle that we can initially set/reset the board to
    puzzle: Option<[[usize; 9]; 9]>,
    /// Solution that we can check the board to
    /// I imagine not all API's or manually setting will supply a solution so we might lean on the
    /// Sudoku Rules to verify that it is proper solution
    solution: Option<[[usize; 9]; 9]>,
    /// Optional Enum to track what difficulty the board is at
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
    pub fn reset(&mut self) {
        if let Some(puzzle) = self.puzzle {
            self.board = puzzle;
        }
    }
    pub fn won(&self) -> bool {
        if let Some(solution) = self.solution {
            return solution == self.board;
        }
        false
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
    #[test]
    fn test_update_cell() {
        let mut sudoku = Sudoku::new();
        let board = [[1; 9]; 9];
        sudoku.set_board(board);
        sudoku.update_cell(0, 0, 2);
        let updated_board = [
            [2, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(sudoku.get_board(), updated_board);
    }
    #[test]
    fn test_reset_with_puzzle() {
        let mut sudoku = Sudoku::new();
        let board = [[1; 9]; 9];
        let puzzle = [[2; 9]; 9];
        sudoku.set_board(board);
        sudoku.set_puzzle(puzzle);
        sudoku.reset();
        assert_eq!(sudoku.get_board(), puzzle);
    }
    #[test]
    fn test_reset_without_puzzle() {
        let mut sudoku = Sudoku::new();
        let board = [[1; 9]; 9];
        sudoku.set_board(board);
        sudoku.reset();
        assert_eq!(sudoku.get_board(), board);
    }
    #[test]
    fn test_won_with_solution() {
        let mut sudoku = Sudoku::new();
        let board = [[1; 9]; 9];
        let solution = [[1; 9]; 9];
        sudoku.set_board(board);
        sudoku.set_solution(solution);
        assert_eq!(sudoku.won(), true);
    }
}
