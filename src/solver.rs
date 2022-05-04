use std::fmt;

#[derive(Clone, Debug)]
pub struct Cell {
    pub value: Option<u8>,
    pub possible_values: Vec<u8>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            value: None,
            possible_values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    fn set_value(&mut self, value: u8) {
        self.value = Some(value);
        self.possible_values.clear();
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: std::iter::repeat_with(Cell::new).take(81).collect(),
        }
    }

    fn set_value(&mut self, row: usize, col: usize, value: u8) {
        let index = (row * 9) + col;
        self.cells[index].set_value(value);
        self.cells[row*9..(row*9)+9].iter_mut().for_each(|cell| {
            if cell.value.is_none() {
                cell.possible_values.retain(|&x| x != value);
            }
        });
        self.cells[col..81].iter_mut().step_by(9).for_each(|cell| {
            if cell.value.is_none() {
                cell.possible_values.retain(|&x| x != value);
            }
        });
        let row_start = (row / 3) * 3;
        let col_start = (col / 3) * 3;
        self.cells[row_start*9+col_start..(row_start+2)*9+col_start+3].chunks_mut(3).step_by(3).for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.value.is_none() {
                    cell.possible_values.retain(|&x| x != value);
                }
            });
        });
    }

    pub fn is_complete(&self) -> bool {
        self.cells.iter().all(|cell| cell.value.is_some())
    }
}

pub struct InitialState {
    known_values: Vec<(usize, usize, u8)>,
}

impl InitialState {
    pub fn new(known_values: Vec<(usize, usize, u8)>) -> Self {
        InitialState {
            known_values,
        }
    }

    fn to_board(&self) -> Board {
        let mut board = Board::new();
        self.known_values.iter().for_each(|(row, col, value)| {
            board.set_value(*row, *col, *value);
        });
        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for col in 0..9 {
                let index = (row * 9) + col;
                let value = self.cells[index].value.unwrap_or(0);
                write!(f, "{}", value)?;
                if col % 3 == 2 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(initial_state: InitialState) -> Option<Board> {
    let board = initial_state.to_board();
    solve_recursive(board)
}

fn solve_recursive(mut board: Board) -> Option<Board> {
    while !board.is_complete() {
        let mut was_change_made = false;
        for cell in board.cells.iter_mut() {
            if cell.possible_values.len() == 1 {
                cell.set_value(cell.possible_values[0]);
                was_change_made = true;
            }
        }

        if !was_change_made {
            for (i, cell) in board.cells.iter().enumerate() {
                for possible_value in cell.possible_values.iter() {
                    let mut board_with_guess = board.clone();
                    board_with_guess.set_value(i / 9, i % 9, *possible_value);
                    if let Some(solved_board) = solve_recursive(board_with_guess) {
                        return Some(solved_board);
                    }
                }
            }

            return None;
        }
    }

    Some(board)
}
