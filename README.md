# wasm-sudoku-rust
Rust library containing WebAssembly source code for Sudoku helper methods.

## Install
These are the steps to configure the project. There are test methods included to ensure everything is functioning as expected.
### Pre-requisites
You need to have [RUST](https://www.rust-lang.org/tools/install) and [WASM](https://rustwasm.github.io/docs/book/game-of-life/setup.html) installed prior running the application. Use the links provided to install the pre-requesites.
### Install Steps
1. Clone the Repository
3. run `cargo test` to verify functionality
4. run `wasm-pack build --target web` to build the binaries
5. copy the binaries into a folder in your site's source code
6. call the methods exposed in the JS to generate, verify, or solve sudoku

## Usage

### Generating a puzzle

