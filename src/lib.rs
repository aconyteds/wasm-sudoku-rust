mod sudoku;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_sudoku(input: &str) -> bool {
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        if c != '.' {
            board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
        }
    }
    sudoku::validate_sudoku(&board)
}

#[wasm_bindgen]
pub fn solve_sudoku(input: &str) -> String {
    // convert the board to a vector of vectors 9x9.
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        if c != '.' {
            board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
        }
    }
    sudoku::solve_sudoku(&mut board, false);
    // convert the board into a string
    let mut output = String::new();
    for i in 0..9 {
        for j in 0..9 {
            output.push_str(&board[i][j].to_string());
        }
    }
    output
}

#[wasm_bindgen]
pub fn generate_sudoku() -> String {
    let board = sudoku::generate_sudoku();
    let mut output = String::new();
    for i in 0..9 {
        for j in 0..9 {
            output.push_str(&board[i][j].to_string());
        }
    }
    output
}
