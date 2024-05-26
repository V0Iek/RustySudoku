use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

fn clear_terminal() {
  execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");

  print!("\x1B[H");
  io::stdout().flush().expect("Failed to flush stdout");
}

fn check_board(board: &[[i8; 9]; 9]) -> bool {
  for row in board.iter() {
    for &cell in row.iter() {
      if cell == 0 {
        return false;
      }
    }
  }
  true
}

fn is_valid(board: &[[i8; 9]; 9], row: usize, col: usize, value: i8) -> bool {
  // Check the row
  for c in 0..9 {
    if board[row][c] == value {
      return false;
    }
  }
  // Check the column
  for r in 0..9 {
    if board[r][col] == value {
      return false;
    }
  }
  // Check the 3x3 subgrid
  let square_row = (row / 3) * 3;
  let square_col = (col / 3) * 3;
  for r in square_row..square_row + 3 {
    for c in square_col..square_col + 3 {
      if board[r][c] == value {
        return false;
      }
    }
  }
  true
}

fn fill_board(board: &mut [[i8; 9]; 9]) -> bool {
  let mut number_list: [i8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut rng = thread_rng();

  for row in 0..9 {
    for col in 0..9 {
      if board[row][col] == 0 {
        number_list.shuffle(&mut rng);

        for &value in number_list.iter() {
          if is_valid(board, row, col, value) {
            board[row][col] = value;

            if check_board(board) || fill_board(board) {
              return true;
            }

            // Backtrack
            board[row][col] = 0;
          }
        }
        return false; // No valid number found, trigger backtracking
      }
    }
  }
  check_board(board)
}

fn draw_board(board: [[i8; 9]; 9]) {
  clear_terminal();

  let mut errs = 0;

  for row in 0..9 {
    if row == 3 || row == 6 {
      println!("------+-------+------");
    }

    for cell in 0..9 {
      if cell == 3 || cell == 6 {
        print!("| ");
      }

      print!("{} ", board[row][cell]);

      if board[row][cell] == 0 {
        errs += 1;
      }
    }

    println!();
  }

  println!("Erroros: {}", errs);
}

fn main() {
  let mut board: [[i8; 9]; 9] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
  ];

  fill_board(&mut board);

  draw_board(board);
}
