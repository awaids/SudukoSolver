use crate::board::Board;

pub struct PrimitiveSolver;
pub struct LeastoptionsFirstSolver;

pub trait SolveSudoku {
    // reqiured trait of the solver
    fn solve(&self, board: &mut Board) -> bool;
}

impl SolveSudoku for PrimitiveSolver {
    fn solve(&self, board: &mut Board) -> bool {
        fn is_safe(row: &[char; 9], col: &[char; 9], grid: &[char; 9], num: char) -> bool {
            !row.contains(&num) && !col.contains(&num) && !grid.contains(&num)
        }

        for row_idx in 0..9 {
            for col_idx in 0..9 {
                if board.get(row_idx, col_idx) == '.' {
                    let (row, col, grid) = board.get_row_col_grid(row_idx, col_idx);
                    for num in '1'..='9' {
                        if is_safe(&row, &col, &grid, num) {
                            board.update_val(row_idx, col_idx, num);
                            println!("Filled in {num} at {row_idx}, {col_idx}");
                        };
                    }
                }
            }
        }
        board.is_valid()
    }
}

#[derive(Debug)]
struct AllowedVal {
    row: usize,
    col: usize,
    val: Vec<char>,
}

impl SolveSudoku for LeastoptionsFirstSolver {
    fn solve(&self, board: &mut Board) -> bool {
        fn allowed_vals(arow: &[char; 9], col: &[char; 9], grid: &[char; 9]) -> Vec<char> {
            // Get all allowed vals for the row/col/grid
            let mut allowed_vals: Vec<char> = Vec::new();
            for val in '1'..='9' {
                if !arow.contains(&val) && !col.contains(&val) && !grid.contains(&val) {
                    allowed_vals.push(val);
                }
            }
            allowed_vals
        }

        fn get_all_possibilities(board: &mut Board) -> Vec<AllowedVal> {
            let mut possibilities: Vec<AllowedVal> = vec![];

            for row_idx in 0..9 {
                for col_idx in 0..9 {
                    if board.get(row_idx, col_idx) == '.' {
                        let (row, col, grid) = board.get_row_col_grid(row_idx, col_idx);
                        let allowed_vals = allowed_vals(&row, &col, &grid);
                        if !allowed_vals.is_empty() {
                            possibilities.push(AllowedVal {
                                row: row_idx,
                                col: col_idx,
                                val: allowed_vals,
                            });
                        }
                    }
                }
            }
            possibilities.sort_by_key(|x| x.val.len());
            possibilities
        }

        let mut possibilities = get_all_possibilities(board);
        while !possibilities.is_empty() {
            // fill value from the top most possibilities
            let least_pos = &possibilities[0];
            board.update_val(least_pos.row, least_pos.col, least_pos.val[0]);
            possibilities = get_all_possibilities(board);
        }

        board.is_valid()
    }
}
