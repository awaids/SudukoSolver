use std::fmt;
use std::usize;

const ORIG_BOARD: [[char; 9]; 9] = [
    ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
];

fn is_row_valid(row: &[char; 9]) -> bool {
    // function to check if the row does not contain any repetiton and only valid numerics
    let allowed_vals = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for val in allowed_vals {
        if !row.contains(&val) {
            return false;
        }
    }
    true
}

pub struct Board {
    board: [[char; 9]; 9],
    orig_board: [[char; 9]; 9],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, " |")?;
                }
                write!(
                    f,
                    " {}",
                    if cell == '0' {
                        '.'
                    } else {
                        cell.to_string().chars().next().unwrap()
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(board: Option<[[char; 9]; 9]>) -> Self {
        match board {
            None => Board {
                board: ORIG_BOARD,
                orig_board: ORIG_BOARD,
            },
            Some(x) => Board {
                board: x,
                orig_board: x,
            },
        }
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        assert!(row < 9 && col < 9);
        self.board[row][col]
    }

    pub fn update_val(&mut self, row: usize, col: usize, val: char) -> bool {
        // returns ture if the value was updated
        assert!(row <= 9 && col <= 9);
        assert!(val.is_numeric());
        let numeric_val = val.to_digit(10).unwrap();
        assert!(numeric_val > 0 && numeric_val < 10);
        match self.orig_board[row][col] {
            '.' => {
                self.board[row][col] = val;
                true
            }
            _ => false,
        }
    }

    pub fn get_row(&self, row_idx: usize) -> [char; 9] {
        assert!(row_idx <= 9);
        self.board[row_idx]
    }

    pub fn get_col(&self, col_idx: usize) -> [char; 9] {
        assert!(col_idx <= 9);
        let col: Vec<char> = (0..9).map(|x| self.board[x][col_idx]).collect();
        col.try_into().unwrap()
    }

    pub fn get_grid(&self, grid_id: (usize, usize)) -> [char; 9] {
        assert!(grid_id.0 < 3 && grid_id.1 < 3);
        let mut grid = ['.'; 9];
        for i in 0..3 {
            for j in 0..3 {
                grid[i * 3 + j] = self.board[grid_id.0 * 3 + i][grid_id.1 * 3 + j];
            }
        }
        grid
    }

    pub fn get_row_col_grid(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> ([char; 9], [char; 9], [char; 9]) {
        let row = self.get_row(row_idx);
        let col = self.get_col(col_idx);
        let grid = self.get_grid((row_idx / 3, col_idx / 3));
        (row, col, grid)
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..9 {
            let (row, col, grid) = self.get_row_col_grid(i, i);

            if !is_row_valid(&row) {
                println!("Row {row:?} Failed");
                return false;
            }

            if !is_row_valid(&col) {
                println!("Col {col:?} Failed");
                return false;
            }

            if !is_row_valid(&grid) {
                println!("Grid {grid:?} Failed");
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod board_test {
    use super::*;

    #[test]
    fn new_board() {
        assert_eq!(Board::new(None).board, ORIG_BOARD);
    }

    #[test]
    fn test_reset() {
        let mut board = Board::new(None);
        board.board[0][0] = '5';
        board.reset();
        assert_eq!(board.board, ORIG_BOARD);
    }

    #[test]
    #[should_panic]
    fn test_update_panic() {
        let mut board = Board::new(None);
        board.update_val(0, 0, 'p');
    }

    #[test]
    fn test_update_false() {
        let mut board = Board::new(None);
        // There is already a value at 0,0
        assert_eq!(board.update_val(0, 2, '1'), true);
        assert_eq!(board.board[0][2], '1');
    }

    #[test]
    fn test_update_true() {
        let mut board = Board::new(None);
        // There is already a value at 0,0
        assert_eq!(board.update_val(0, 0, '1'), false);
    }

    #[test]
    fn test_getrow() {
        let board = Board::new(None);
        // There is already a value at 0,0
        assert_eq!(
            board.get_row(0),
            ['5', '3', '.', '.', '7', '.', '.', '.', '.']
        );
    }

    #[test]
    fn test_getcol() {
        let board = Board::new(None);
        // There is already a value at 0,0
        assert_eq!(
            board.get_col(0),
            ['5', '6', '.', '8', '4', '7', '.', '.', '.']
        );
    }

    #[test]
    fn test_get_3x3_grid() {
        let board = Board::new(None);
        assert_eq!(
            board.get_grid((0, 0)),
            ['5', '3', '.', '6', '.', '.', '.', '9', '8',]
        );

        assert_eq!(
            board.get_grid((2, 2)),
            ['2', '8', '.', '.', '.', '5', '.', '7', '9']
        );
    }

    #[test]
    fn test_get_row_col_grid() {
        let board = Board::new(None);
        let (row, col, grid) = board.get_row_col_grid(8, 8);

        assert_eq!(row, ['.', '.', '.', '.', '8', '.', '.', '7', '9']);
        assert_eq!(col, ['.', '.', '.', '3', '1', '6', '.', '5', '9']);
        assert_eq!(grid, ['2', '8', '.', '.', '.', '5', '.', '7', '9']);
    }

    #[test]
    fn test_valid_1() {
        let valid_board: [[char; 9]; 9] = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let board = Board::new(Some(valid_board));
        // There is already a value at 0,0
        assert_eq!(board.is_valid(), true);
    }

    #[test]
    fn test_invalid_1() {
        let valid_board: [[char; 9]; 9] = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', 'X'],
        ];

        let board = Board::new(Some(valid_board));
        // There is already a value at 0,0
        assert_eq!(board.is_valid(), false);
    }

    #[test]
    fn test_invalid_2() {
        let valid_board: [[char; 9]; 9] = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '9'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let board = Board::new(Some(valid_board));
        // There is already a value at 0,0
        assert_eq!(board.is_valid(), false);
    }
}
