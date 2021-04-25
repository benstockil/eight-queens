use std::env;
use itertools::Itertools;

// Increment the state of the board, finding the next permutation
fn inc_board(board: &mut [u8; 8]) {
    for i in 0..8 {
        if board[i] == 7 {
            board[i] = 0;
        } else {
            board[i] += 1;
            break;
        }
    }
}

// Verify that all pieces of the board are on different rows
fn all_unique(board: &[u8; 8]) -> bool {
    for i in 0..7 {
        for j in i + 1..8 {
            if board[i] == board[j] {
                return false;
            }
        }
    }
    return true;
}

// Verify that none of the pieces are on the same diagonal
fn check_diags(board: &[u8; 8]) -> bool {
    for i in 0..7 {
        for j in i + 1..8 {
            let x_diff = (board[i] - board[j]) as i8;
            let y_diff = (i - j) as i8;
            if x_diff.abs() == y_diff.abs() {
                return false;
            }
        }
    }
    return true;
}

// Convert digit of board state to graphical row
fn to_graphical(digit: &u8) -> String {
    let mut row = ['.'; 8];
    row[*digit as usize] = '#';
    row.iter().intersperse(&' ').collect::<String>()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&String::from("-v"));

    // List of boards which satisfy the 8 queens problem
    let mut good_boards = Vec::<[u8; 8]>::new();
    // Board to iterate over
    let mut board = [0; 8];

    println!("Finding boards...");
    // Until board reaches final permutation
    while board != [7 as u8; 8] {
        // Go to next permutation
        inc_board(&mut board);
        // If the board satisfies the 8 queens problem
        if all_unique(&board) && check_diags(&board) {
            // Push to list of good boards
            good_boards.push(board.clone());
        }
    }

    // Output
    println!("Found {} boards.", good_boards.len());
    let mut board_num = 0;
    good_boards.reverse();
    while let Some(mut good_board) = good_boards.pop() {
        good_board.reverse();
        board_num += 1;
        let fmt_board = good_board
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<String>();
        println!("Board {}: {}", board_num, fmt_board);

        // Print graphical representations
        if verbose {
            for digit in good_board.iter() {
                println!("{}", to_graphical(digit));
            }
            println!("");
        }
    }
}