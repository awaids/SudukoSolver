# SudukoSolver

My first attempt at Rust. This project includes two simple solvers to solve a predefined Sudoku puzzle.

## Setup

1. Ensure you have Rust installed. If not, download and install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/SudukoSolver.git
    ```
3. Navigate to the project directory:
    ```sh
    cd SudukoSolver
    ```

## Usage

To run the solver, use the following command:
```sh
cargo run
```

## Solvers

The project includes two solvers:
1. `PrimitiveSolver`: A basic solver.
2. `LeastoptionsFirstSolver`: A solver that prioritizes cells with the fewest options.

You can switch between them by commenting/uncommenting the appropriate lines in `main.rs`:
```rust
let solver: solvers::PrimitiveSolver = solvers::PrimitiveSolver;
// let solver: solvers::LeastoptionsFirstSolver = solvers::LeastoptionsFirstSolver;
```

## Example

Here is an example of running the solver:
```sh
cargo run
```

Output for the `PrimitiveSolver`:
```
Initial Board:
 5 3 . | . 7 . | . . .
 6 . . | 1 9 5 | . . .
 . 9 8 | . . . | . 6 .
------+-------+------
 8 . . | . 6 . | . . 3
 4 . . | 8 . 3 | . . 1
 7 . . | . 2 . | . . 6
------+-------+------
 . 6 . | . . . | 2 8 .
 . . . | 4 1 9 | . . 5
 . . . | . 8 . | . 7 9

Couldn't Solve Puzzle
 5 3 4 | 6 7 8 | 9 2 .
 6 7 2 | 1 9 5 | 8 4 .
 1 9 8 | 3 4 2 | 7 6 .
------+-------+------
 8 5 9 | 7 6 4 | . . 3
 4 2 6 | 8 5 3 | . 9 1
 7 1 3 | 9 2 . | 5 . 6
------+-------+------
 9 6 7 | 5 3 . | 2 8 4
 3 8 . | 4 1 9 | 6 . 5
 2 4 5 | . 8 6 | 3 7 9
```
Similarly, output of the `LeastoptionsFirstSolver`:
```
 5 3 . | . 7 . | . . .
 6 . . | 1 9 5 | . . .
 . 9 8 | . . . | . 6 .
------+-------+------
 8 . . | . 6 . | . . 3
 4 . . | 8 . 3 | . . 1
 7 . . | . 2 . | . . 6
------+-------+------
 . 6 . | . . . | 2 8 .
 . . . | 4 1 9 | . . 5
 . . . | . 8 . | . 7 9

Solved Puzzle
 5 3 4 | 6 7 8 | 9 1 2
 6 7 2 | 1 9 5 | 3 4 8
 1 9 8 | 3 4 2 | 5 6 7
------+-------+------
 8 5 9 | 7 6 1 | 4 2 3
 4 2 6 | 8 5 3 | 7 9 1
 7 1 3 | 9 2 4 | 8 5 6
------+-------+------
 9 6 1 | 5 3 7 | 2 8 4
 2 8 7 | 4 1 9 | 6 3 5
 3 4 5 | 2 8 6 | 1 7 9
```