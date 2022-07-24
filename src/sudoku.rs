use rand::random;

fn is_valid(board: &Vec<Vec<i8>>, row: usize, col: usize, ch: i8) -> bool {
    // check if the value already exists in the row
    for i in 0..9 {
        if i != col && board[row][i] == ch {
            return false;
        }
    }

    // check if the value already exists in the column
    for i in 0..9 {
        if i != row && board[i][col] == ch {
            return false;
        }
    }

    // check if the value already exists in the 3x3 box
    let box_i = (row / 3) * 3;
    let box_j = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if row != i + box_i && col != j + box_j && board[i + box_i][j + box_j] == ch {
                return false;
            }
        }
    }
    true
}

// Method to validate a sudoku board is solvable
pub fn validate_sudoku(board: &Vec<Vec<i8>>) -> bool {
    // check if the board is valid
    if board.len() != 9 || board[0].len() != 9 {
        return false;
    }
    // check if the board is solvable
    for i in 0..9 {
        for j in 0..9 {
            let value = board[i][j];
            if value == 0 {
                continue;
            }
            if !is_valid(board, i, j, value) {
                return false;
            }
        }
    }
    true
}

// write a method to solve sudoku puzzles.
// you may assume that the board is valid and is solvable.
pub fn solve_sudoku(board: &mut Vec<Vec<i8>>, reverse: bool) -> bool {
    // Modifies the board directly
    // Returns true if the board is solved, false otherwise.
    solve_iteratively(board, reverse)
}

fn solve_iteratively(board: &mut Vec<Vec<i8>>, reverse: bool) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut unsolved_indexes = vec![];
    let mut backtrack = false;
    'outer: loop {
        if col == board[row].len() {
            // Continue to the next row
            col = 0;
            row += 1;
            if row == board.len() {
                break 'outer;
            }
            continue 'outer;
        }
        if board[row][col] == 0 || backtrack {
            let mut start = if reverse { 9 } else { 1 };
            let end = if reverse { 0 } else { 10 };
            if backtrack {
                start = board[row][col];
            }

            'check_values: while start != end {
                let i = start;
                start += if reverse { -1 } else { 1 };
                println!("{} {} {}", row, col, i);
                if i == 0 || (backtrack && i == board[row][col]) {
                    continue 'check_values;
                }
                if is_valid(board, row, col, i) {
                    backtrack = false;
                    unsolved_indexes.push((row, col));
                    board[row][col] = i;
                    col += 1;
                    continue 'outer;
                }
            }
            if unsolved_indexes.len() == 0 {
                println!("Invalid Sudoku");
                return false;
            }
            // backtrack
            board[row][col] = 0;
            (row, col) = unsolved_indexes.pop().unwrap();
            backtrack = true;
            continue 'outer;
        }
        col += 1;
    }
    true
}

// Create a method to randomly genereate a sudoku puzzle.
// The method should return a 2D filled with 0-9 and 0.
// The 2D array should be 9x9.
pub fn generate_sudoku() -> Vec<Vec<i8>> {
    let mut board = vec![vec![0; 9]; 9];
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // fill the first row with 1-9 randomly
    for i in 0..9 {
        let index = random::<usize>() % values.len();
        board[0][i] = values[index];
        // remove the value from the list so it won't be used again
        values.remove(index);
    }
    // fill the first column with 1-9 randomly which does not appear in the first row
    let mut column_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    //remove the value from 0,0 to avoid duplicates
    column_values.retain(|&x| x != board[0][0]);
    for i in 1..9 {
        let mut invalid = true;
        let mut index = random::<usize>() % column_values.len();
        while invalid {
            let value = column_values[index];
            if !is_in_box(&board, i, 0, value) {
                invalid = false;
                board[i][0] = value;
                column_values.remove(index);
            } else {
                index = random::<usize>() % column_values.len();
            }
        }
    }
    solve_sudoku(&mut board, true);
    board
}

fn generate_cell(board: &mut Vec<Vec<i8>>, row: usize, col: usize) {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // filter the values for only what is valid for the current position
    let valid_values = values
        .into_iter()
        .filter(|&x| is_valid(board, row, col, x))
        .collect::<Vec<i8>>();
    // if the list of values is empty, then we need to backtrack

    if valid_values.len() == 0 {
        board[row][col] = 0;
        return;
    }
    // get a random value from the valid values
    let value = valid_values[random::<usize>() % valid_values.len()];
    board[row][col] = value;
}

fn is_in_row(board: &Vec<Vec<i8>>, row: usize, value: i8) -> bool {
    for i in 0..9 {
        if board[row][i] == value {
            return true;
        }
    }
    false
}

fn is_in_col(board: &Vec<Vec<i8>>, col: usize, value: i8) -> bool {
    for i in 0..9 {
        if board[i][col] == value {
            return true;
        }
    }
    false
}

fn is_in_box(board: &Vec<Vec<i8>>, row: usize, col: usize, value: i8) -> bool {
    let row_start = (row / 3) * 3;
    let col_start = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[row_start + i][col_start + j] == value {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn correctly_validates_board() {
        // create a sudoku puzzle
        let input = [
            [7, 8, 0, 4, 0, 0, 1, 2, 0],
            [6, 0, 0, 0, 7, 5, 0, 0, 9],
            [0, 0, 0, 6, 0, 1, 0, 7, 8],
            [0, 0, 7, 0, 4, 0, 2, 6, 0],
            [0, 0, 1, 0, 5, 0, 9, 3, 0],
            [9, 0, 4, 0, 6, 0, 0, 0, 5],
            [0, 7, 0, 3, 0, 0, 0, 1, 2],
            [1, 2, 0, 0, 0, 7, 4, 0, 0],
            [0, 4, 9, 2, 0, 6, 0, 0, 7],
        ];
        // convert the input into a Vec<Vec<i8>>
        let mut board = vec![vec![0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = input[i][j];
            }
        }

        assert_eq!(
            super::validate_sudoku(&board),
            true,
            "Validation method failed."
        );
    }
    #[test]
    fn correctly_solves_sudoku() {
        // create a sudoku puzzle
        let input = [
            [7, 8, 0, 4, 0, 0, 1, 2, 0],
            [6, 0, 0, 0, 7, 5, 0, 0, 9],
            [0, 0, 0, 6, 0, 1, 0, 7, 8],
            [0, 0, 7, 0, 4, 0, 2, 6, 0],
            [0, 0, 1, 0, 5, 0, 9, 3, 0],
            [9, 0, 4, 0, 6, 0, 0, 0, 5],
            [0, 7, 0, 3, 0, 0, 0, 1, 2],
            [1, 2, 0, 0, 0, 7, 4, 0, 0],
            [0, 4, 9, 2, 0, 6, 0, 0, 7],
        ];
        // convert the input into a Vec<Vec<i8>>
        let mut board = vec![vec![0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = input[i][j];
            }
        }
        // solve the sudoku puzzle
        super::solve_sudoku(&mut board, true);
        // Convert the result back into a 2D array
        let mut output: [[i8; 9]; 9] = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                output[i][j] = board[i][j];
            }
        }

        let solution = [
            [7, 8, 5, 4, 3, 9, 1, 2, 6],
            [6, 1, 2, 8, 7, 5, 3, 4, 9],
            [4, 9, 3, 6, 2, 1, 5, 7, 8],
            [8, 5, 7, 9, 4, 3, 2, 6, 1],
            [2, 6, 1, 7, 5, 8, 9, 3, 4],
            [9, 3, 4, 1, 6, 2, 7, 8, 5],
            [5, 7, 8, 3, 9, 4, 6, 1, 2],
            [1, 2, 6, 5, 8, 7, 4, 9, 3],
            [3, 4, 9, 2, 1, 6, 8, 5, 7],
        ];

        assert_eq!(solution, output);
    }
    #[test]
    fn correctly_solves_sudoku_in_reverse() {
        // create a sudoku puzzle
        let input = [
            [7, 8, 0, 4, 0, 0, 1, 2, 0],
            [6, 0, 0, 0, 7, 5, 0, 0, 9],
            [0, 0, 0, 6, 0, 1, 0, 7, 8],
            [0, 0, 7, 0, 4, 0, 2, 6, 0],
            [0, 0, 1, 0, 5, 0, 9, 3, 0],
            [9, 0, 4, 0, 6, 0, 0, 0, 5],
            [0, 7, 0, 3, 0, 0, 0, 1, 2],
            [1, 2, 0, 0, 0, 7, 4, 0, 0],
            [0, 4, 9, 2, 0, 6, 0, 0, 7],
        ];
        // convert the input into a Vec<Vec<i8>>
        let mut board = vec![vec![0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = input[i][j];
            }
        }
        // solve the sudoku puzzle
        super::solve_iteratively(&mut board, true);
        // Convert the result back into a 2D array
        let mut output: [[i8; 9]; 9] = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                output[i][j] = board[i][j];
            }
        }

        let solution = [
            [7, 8, 5, 4, 3, 9, 1, 2, 6],
            [6, 1, 2, 8, 7, 5, 3, 4, 9],
            [4, 9, 3, 6, 2, 1, 5, 7, 8],
            [8, 5, 7, 9, 4, 3, 2, 6, 1],
            [2, 6, 1, 7, 5, 8, 9, 3, 4],
            [9, 3, 4, 1, 6, 2, 7, 8, 5],
            [5, 7, 8, 3, 9, 4, 6, 1, 2],
            [1, 2, 6, 5, 8, 7, 4, 9, 3],
            [3, 4, 9, 2, 1, 6, 8, 5, 7],
        ];

        assert_eq!(solution, output);
    }
    #[test]
    fn correctly_generates_sudoku() {
        let mut board = super::generate_sudoku();
        super::solve_sudoku(&mut board, false);
        let mut output: [[i8; 9]; 9] = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                output[i][j] = board[i][j];
            }
        }

        assert_eq!(
            super::validate_sudoku(&board),
            true,
            "Sudoku board is not valid \n {}",
            board
                .into_iter()
                .map(|row| row
                    .into_iter()
                    .map(|col| col.to_string())
                    .collect::<Vec<String>>()
                    .join(","))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
