use reqwest::blocking::{Client, get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct HttpYouDoSudokuRequest {
    difficulty: String,
    solution: bool,
    array: bool,
}
#[derive(Debug, Deserialize)]
struct HttpYouDoSudokuResponse {
    difficulty: String,
    puzzle: String,
    solution: String,
}

pub fn get_board(difficulty: String) -> ([[usize; 9]; 9], [[usize; 9]; 9]) {
    let client = Client::new();
    let request_body = HttpYouDoSudokuRequest {
        difficulty: difficulty.to_lowercase(),
        solution: true,
        array: false,
    };
    let response: HttpYouDoSudokuResponse = client
        .post("https://youdosudoku.com/api")
        .json(&request_body)
        .send()
        .unwrap()
        .json()
        .unwrap();

    //println!("{:#?}", response);

    // Since YouDoSudoku encodes their puzzles as a single String with 81 characters,
    // we will have to do some manual parsing which we can take the chars ascii equivalent since its just numbers.
    // They do have support for arrays but i'll
    // explore that in the future.
    let mut puzzle = [[0; 9]; 9];
    let mut solution = [[0; 9]; 9];
    for (i, c) in response.puzzle.chars().enumerate() {
        let row = i / 9 as usize;
        let col = i % 9 as usize;
        let digit = c as usize - '0' as usize;
        puzzle[row][col] = digit;
    }
    for (i, c) in response.solution.chars().enumerate() {
        let row = i / 9 as usize;
        let col = i % 9 as usize;
        solution[row][col] = c as usize - '0' as usize;
    }
    (puzzle, solution)
}
