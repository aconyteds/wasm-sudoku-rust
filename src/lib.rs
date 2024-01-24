mod sudoku;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_sudoku(input: &str) -> bool {
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
    }
    sudoku::validate_sudoku(&board)
}

#[wasm_bindgen]
pub fn solve_sudoku(input: &str) -> String {
    // convert the board to a vector of vectors 9x9.
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
    }
    let solved = sudoku::solve_sudoku(&mut board, false);
    if solved {
        // convert the board into a string
        let mut output = String::new();
        for i in 0..9 {
            for j in 0..9 {
                output.push_str(&board[i][j].to_string());
            }
        }
        output
    } else {
        "Invalid Puzzle Provided".to_string()
    }
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

#[wasm_bindgen]
pub fn generate_suggestions(input: &str, row: usize, col: usize) -> String {
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
    }
    let mut output = String::new();
    let suggestions = sudoku::get_suggestions(&board, row, col);
    suggestions
        .iter()
        .for_each(|x| output.push_str(&x.to_string()));
    output
}

#[wasm_bindgen]
pub fn generate_all_suggestions(input: &str) -> String {
    let mut board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
    for (i, c) in input.chars().enumerate() {
        board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
    }
    // Output is a JSON array of arrays
    let mut output = String::new();
    output.push_str(&'['.to_string());
    for i in 0..9 {
        for j in 0..9 {
            let suggestions = sudoku::get_suggestions(&board, i, j);
            // Need to add array designator to the output
            output.push_str(&'['.to_string());
            output.push_str(
                &suggestions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            output.push_str(&']'.to_string());
            if i != 8 && j != 8 {
                output.push_str(&','.to_string());
            }
        }
    }
    output.push_str(&']'.to_string());
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wasm_can_validate_sudoku() {
        let input =
            "071009045596800001040051986389006400004000097012493650168020073437065020900300004";
        assert_eq!(validate_sudoku(input), true);
        let input =
            "771009045596800001040051986389006400004000097012493650168020073437065020900300004";
        assert_eq!(validate_sudoku(input), false);
    }
    #[test]
    fn wasm_can_solve_sudoku() {
        let input =
            "780400120600075009000601078007040260001050930904060005070300012120007400049206007";
        let solution =
            "785439126612875349493621578857943261261758934934162785578394612126587493349216857";
        assert_eq!(solve_sudoku(input), solution);
        let input =
            "771009045596800001040051986389006400004000097012493650168020073437065020900300004";
        assert_eq!(solve_sudoku(input), "Invalid Puzzle Provided");
    }
    #[test]
    fn wasm_can_generate_sudoku() {
        let mut test_count = 0;
        while test_count < 100 {
            let board = generate_sudoku();
            assert_eq!(board.len(), 81, "Incorrect Board length Generated");
            assert_eq!(validate_sudoku(&board), true, "Board is not valid");
            let mut rust_board: Vec<Vec<i8>> = vec![vec![0; 9]; 9];
            for (i, c) in board.chars().enumerate() {
                rust_board[i / 9][i % 9] = c.to_digit(10).unwrap() as i8;
            }
            assert_eq!(
                sudoku::solve_sudoku(&mut rust_board.clone(), false),
                sudoku::solve_sudoku(&mut rust_board, true),
                "Solutions do not match"
            );
            test_count += 1;
        }
    }
    #[test]
    fn wasm_can_generate_suggestions() {
        let input =
            "071009045596800001040051986389006400004000097012493650168020073437065020900300004";
        let suggestions = generate_suggestions(input, 0, 0);
        assert_eq!(suggestions, "28");
        let suggestions = generate_suggestions(input, 1, 3);
        assert_eq!(suggestions, "278");
    }
    #[test]
    fn wasm_can_generate_all_suggestions() {
        let input =
            "071009045596800001040051986389006400004000097012493650168020073437065020900300004";
        let suggestions = generate_all_suggestions(input);
        assert_eq!(suggestions, "[[2,8],[2,7],[1,3],[2,6],[3],[2,9],[2,3],[3,4],[2,5][2,5],[2,9],[3,6],[2,7,8],[3,4,7],[2,4,7],[2,3,7],[3],[1,2][2],[2,4],[3],[2,7],[3,5,7],[1,2,7],[2,3,7,9],[3,8],[2,6][3,7],[5,8],[5,9],[1,2,5,7],[1,7],[2,6,7],[1,2,4],[1],[2][6],[5],[4,5],[1,2,5],[1,8],[2,8],[1,2,3,8],[1,3,9],[2,7,8][7],[1],[2],[4,7],[7,8,9],[3,7,8],[6,8],[5],[8][1],[5,6],[5,8],[9],[2,4],[4],[5],[7],[3,9][4],[3],[7],[1,9],[1,6,8],[5,8],[1,8],[1,2],[8,9][2,9][2,5][5][1,3,7][1,7,8][7,8][1,5,8][1,6][4,8]]");
    }
}
