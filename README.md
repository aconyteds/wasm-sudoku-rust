## wasm-sudoku-rust
Rust library containing WebAssembly source code for Sudoku helper methods.

# Install
These are the steps to configure the project. There are test methods included to ensure everything is functioning as expected.
### Pre-requisites
You need to have [RUST](https://www.rust-lang.org/tools/install) and [WASM](https://rustwasm.github.io/docs/book/game-of-life/setup.html) installed prior running the application. Use the links provided to install the pre-requesites.
### Install Steps
1. Clone the Repository
3. run `cargo test` to verify functionality
4. run `wasm-pack build --target web` to build the binaries
5. copy the binaries into a folder in your site's source code
6. call the methods exposed in the JS to generate, verify, or solve sudoku

# Usage
To use the WASM package you need to migrate the built code into your projects source code. For instance, you could have a `src/WASM/sudoku` folder which holds the contents of the built output. Once you decide where to put the files, then you just need to leverage the methods themselves.

## `generate_sudoku`
To generate a puzzle, the method `generate_sudoku` can be used.
### Inputs
None
### Outputs:: `string`;
`String`: valid puzzle string with 0's for all of the empty squares.

### Example Usage:
```TypeScript
import init, {generate_sudoku} from "./path/to/sudoku/folder";

const generatePuzzle = async (): Promise<number[]> => {
    // Wait for the WASM to be loaded
    await init();
    const newPuzzle: string = generate_sudoku();
    return newPuzzle.split("").map(Number);
}
```
---
## `solve_sudoku`
This method will solve an uncompleted puzzle for you. If the puzzle cannot be completed (because an invalid value was provided), it will return "Invalid Puzzle Provided" as the response.

### Inputs:: Puzzle: `string`;
`Puzzle`: Puzzle string to be solved, should be a string with a length of 81. All characters should be numeric.

### Outputs:: `string`;
`String`: Solved puzzle, or "Invalid Puzzle Provided"

### Example Usage:
```TypeScript
import init, {solve_sudoku} from "./path/to/sudoku/folder";

const solveSudoku = async (puzzle: string): Promise<number[] | false> => {
    // Wait for the WASM to be loaded
    await init();
    const solvedPuzzle = solve_sudoku(puzzle);
    if(solvedPuzzle === "Invalid Puzzle Provided") {
        return false;
    }
    return solvedPuzzle.split("").map(Number);
}
```
---
## `validate_sudoku`
Looks at a sudoku board and validates that it is currently valid. **Note** This does not check whether it can be solved, just that there are no current invalid inputs.

### Inputs:: Puzzle: `string`;
`Puzzle`: Puzzle string to be validated, should be a string with a length of 81. All characters should be numeric.

### Outputs
`bool`: Returns `true` or `false` depending on whether the puzzle is valid.

### Example Usage:
```TypeScript
import init, {validate_sudoku} from "./path/to/sudoku/folder";

const validateSudoku = async (puzzle: string): Promise<boolean> => {
    // Wait for the WASM to be loaded
    await init();
    return validate_sudoku(puzzle);
}
```
---
## `generate_suggestions`
This method will look at a specific cell in the board, and return a string containing the valid candidates for the cell. This is useful if your user's need a hint, or if you don't want to analyze the board yourself.

### Inputs:: Puzzle: `string`, Row: `number`, Col: `number`;
`Puzzle`: Current Puzzle, should be a string with a length of 81. All characters should be numeric.

`Row`: Number between 0 and 8, represents the current row.

`Col`: Number between 0 and 8, represents the current column.

### Outputs:: `string`;
`String`: String with numeric values that are viable candidates for the current cell.

### Example Usage:
```TypeScript
import init, {generate_suggestions} from "./path/to/sudoku/folder";

const getSuggestions = async (puzzle: string, row: number, col: number): Promise<number[]> => {
    // Wait for the WASM to be loaded
    await init();
    const results = generate_suggestions(puzzle, row, col);
    return results.split("").map(Number);
}
```
# More information
You might be wondering, "Why do these methods take strings, and not arrays?". I'll be honest, I was confused too. Rust wouldn't compile when I had the methods accepting Matrices or Arrays, so I had to code it to accept a string instead. The conversions are done in the Rust codebase so that the data can be manipulated properly in the WASM code.

You may want some helper methods to facilitate the use of these libraries though. Depending on how you are working with the data (matrices or arrays) the following methods might be helpful:

## `boardToString`
Converts a Matrix (array of arrays) to a string.
```TypeScript
const boardToString = (board: number[][]): string => board.map((row) => row.join("")).join("");
```
## `boardToArray`
Converts a Matrix (array of arrays) to a single dimensional array.
```TypeScript
const boardToArray = (board: number[][]): number[] => board.flat();
```

## `stringToArray`
Converts a valid board string to an array.
```TypeScript
const stringToArray = (puzzle: string): number[] => puzzle.split("").map(Number);
```

## `stringToBoard`
Converts a valid board string, into a Matrix (array of arrays).
```TypeScript
const stringToBoard = (puzzle: string): number[][] => {
    return puzzle.split("").map(Number).reduce((board, char, indx) => {
      return (indx % 9 === 0 ? board.push([char]) : board[board.length - 1].push(char)) && board
    }, []);
}
```