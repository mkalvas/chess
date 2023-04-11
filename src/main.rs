#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::{io::BufRead, process::exit};

// "◻️♜ ◼️♞ ◻️♝ ◼️♛ ◻️♚ ◼️♝ ◻️♞ ◼️♜"
// "◼️♟ ◻️♟ ◼️♟ ◻️♟ ◼️♟ ◻️♟ ◼️♟ ◻️♟"
// "◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️ "
// "◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️ "
// "◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️ "
// "◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️ "
// "◻️♙ ◼️♙ ◻️♙ ◼️♙ ◻️♙ ◼️♙ ◻️♙ ◼️♙"
// "◼️♖ ◻️♘ ◼️♗ ◻️♕ ◼️♔ ◻️♗ ◼️♘ ◻️♖"

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let mut stdin = std::io::stdin().lock().lines();
    let mut board = chess::Board::new();
    loop {
        print!("{board}");

        if let Some(result) = stdin.next() {
            match result {
                Err(e) => {
                    println!("couldn't read move input: {e}");
                    exit(1);
                }
                Ok(mv) => {
                    let result = board.mv(mv);
                    print!("\x1B[2J\x1B[1;1H");
                    if result.is_err() {
                        println!("invalid move");
                    };
                }
            }
        }
    }
}
