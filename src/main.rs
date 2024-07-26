use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
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

fn solve_board(board: &mut [[i8; 9]; 9], counter: &mut i32) -> bool {
  let mut number_list: [i8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut rng = thread_rng();

  for row in 0..9 {
    for col in 0..9 {
      if board[row][col] == 0 {
        number_list.shuffle(&mut rng);

        for &value in number_list.iter() {
          if is_valid(board, row, col, value) {
            board[row][col] = value;
            *counter += 1;

            if check_board(board) || solve_board(board, counter) {
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

fn remove_numbers(board: &mut [[i8; 9]; 9]) {
  let mut rng = rand::thread_rng();

  for _ in 0..5 {
    let mut row = rng.gen_range(0..9);
    let mut col = rng.gen_range(0..9);

    while board[col][row] == 0 {
      col = rng.gen_range(0..9);
      row = rng.gen_range(0..9);
    }

    board[col][row] = 0;
  }
}

fn draw_board(board: [[i8; 9]; 9]) {
  clear_terminal();

  let mut zeros = 0;

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
        zeros += 1;
      }
    }

    println!();
  }

  println!("Zeros: {}", zeros);
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

  let mut counter = 0;

  solve_board(&mut board, &mut counter);
  remove_numbers(&mut board);
  draw_board(board);
}
