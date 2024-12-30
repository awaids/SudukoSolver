mod board;
mod solvers;

fn run(board: &mut board::Board, solver: &impl solvers::SolveSudoku) -> bool {
    solver.solve(board)
}

fn main() {
    let mut board = board::Board::new(None);
    // let p_solver: solvers::PrimitiveSolver = solvers::PrimitiveSolver;
    let q_solver: solvers::LeastoptionsFirstSolver = solvers::LeastoptionsFirstSolver;

    println!("{}", board);

    match run(&mut board, &q_solver) {
        true => println!("Solved Puzzle"),
        false => println!("Couldn't Solve Puzzle"),
    };

    println!("{}", board);
}
