use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, io::Write};

lazy_static! {
    static ref MOVE_FORMAT: Regex = Regex::new(r"^[a-zA-Z]\d[a-zA-Z]\d$").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Self::Black => "◼️",
            Self::White => "◻️",
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    None,
    WhitePawn,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    BlackPawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let glyph = match self {
            Self::None => " ",
            Self::WhitePawn => "♙",
            Self::WhiteKing => "♔",
            Self::WhiteQueen => "♕",
            Self::WhiteRook => "♖",
            Self::WhiteBishop => "♗",
            Self::WhiteKnight => "♘",
            Self::BlackPawn => "♟",
            Self::BlackKing => "♚",
            Self::BlackQueen => "♛",
            Self::BlackRook => "♜",
            Self::BlackBishop => "♝",
            Self::BlackKnight => "♞",
        };
        write!(f, "{}", glyph)
    }
}

#[derive(Debug)]
struct Square {
    coord: &'static str,
    color: Color,
    piece: Piece,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color, self.piece)
    }
}

#[derive(Debug)]
pub struct Board {
    ranks: [[Square; 8]; 8],
    turn: i16,
}

impl Board {
    pub fn new() -> Self {
        INIT_BOARD
    }

    pub fn mv(&mut self, mv: String) -> Result<(), &str> {
        if !MOVE_FORMAT.is_match(&mv) {
            return Err("invalid move");
        }

        let from = &mv[0..2].to_ascii_lowercase();
        let to = &mv[2..].to_ascii_lowercase();

        let mut held = Piece::None;
        for rank in &mut self.ranks {
            for mut square in rank {
                if square.coord == from {
                    held = square.piece;
                    square.piece = Piece::None
                }
            }
        }

        for rank in &mut self.ranks {
            for square in rank {
                if square.coord == to {
                    square.piece = held;
                }
            }
        }

        self.turn += 1;
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let player = match self.turn % 2 == 0 {
            true => "Black",
            false => "White",
        };

        write!(f, "┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n")?;
        write!(f, "┃         CLI Chess        ┃\n")?;
        write!(f, "┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫\n")?;
        write!(f, "┃   a  b  c  d  e  f  g  h ┃\n")?;

        let mut rank_num = 8;
        for rank in self.ranks.iter() {
            write!(f, "┃{} ", rank_num)?;
            rank_num -= 1;

            for square in rank {
                write!(f, "{} ", square)?;
            }

            write!(f, "┃\n")?;
        }

        write!(f, "┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛\n")?;
        write!(f, "{} to play: ", player)?;
        match std::io::stdout().flush() {
            Err(_) => Err(std::fmt::Error),
            Ok(_) => Ok(()),
        }
    }
}

// "◻️♜ ◼️♞ ◻️♝ ◼️♛ ◻️♚ ◼️♝ ◻️♞ ◼️♜"
// "◼️♟ ◻️♟ ◼️♟ ◻️♟ ◼️♟ ◻️♟ ◼️♟ ◻️♟"
// "◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️ "
// "◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️ "
// "◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️ "
// "◼️  ◻️  ◼️  ◻️  ◼️  ◻️  ◼️  ◻️ "
// "◻️♙ ◼️♙ ◻️♙ ◼️♙ ◻️♙ ◼️♙ ◻️♙ ◼️♙"
// "◼️♖ ◻️♘ ◼️♗ ◻️♕ ◼️♔ ◻️♗ ◼️♘ ◻️♖"
pub const INIT_BOARD: Board = Board {
    turn: 1,
    ranks: [
        [
            Square {
                coord: "a8",
                color: Color::White,
                piece: Piece::BlackRook,
            },
            Square {
                coord: "b8",
                color: Color::Black,
                piece: Piece::BlackKnight,
            },
            Square {
                coord: "c8",
                color: Color::White,
                piece: Piece::BlackBishop,
            },
            Square {
                coord: "d8",
                color: Color::Black,
                piece: Piece::BlackQueen,
            },
            Square {
                coord: "e8",
                color: Color::White,
                piece: Piece::BlackKing,
            },
            Square {
                coord: "f8",
                color: Color::Black,
                piece: Piece::BlackBishop,
            },
            Square {
                coord: "g8",
                color: Color::White,
                piece: Piece::BlackKnight,
            },
            Square {
                coord: "h8",
                color: Color::Black,
                piece: Piece::BlackRook,
            },
        ],
        [
            Square {
                coord: "a7",
                color: Color::Black,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "b7",
                color: Color::White,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "c7",
                color: Color::Black,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "d7",
                color: Color::White,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "e7",
                color: Color::Black,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "f7",
                color: Color::White,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "g7",
                color: Color::Black,
                piece: Piece::BlackPawn,
            },
            Square {
                coord: "h7",
                color: Color::White,
                piece: Piece::BlackPawn,
            },
        ],
        [
            Square {
                coord: "a6",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "b6",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "c6",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "d6",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "e6",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "f6",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "g6",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "h6",
                color: Color::Black,
                piece: Piece::None,
            },
        ],
        [
            Square {
                coord: "a5",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "b5",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "c5",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "d5",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "e5",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "f5",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "g5",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "h5",
                color: Color::White,
                piece: Piece::None,
            },
        ],
        [
            Square {
                coord: "a4",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "b4",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "c4",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "d4",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "e4",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "f4",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "g4",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "h4",
                color: Color::Black,
                piece: Piece::None,
            },
        ],
        [
            Square {
                coord: "a3",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "b3",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "c3",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "d3",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "e3",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "f3",
                color: Color::White,
                piece: Piece::None,
            },
            Square {
                coord: "g3",
                color: Color::Black,
                piece: Piece::None,
            },
            Square {
                coord: "h3",
                color: Color::White,
                piece: Piece::None,
            },
        ],
        [
            Square {
                coord: "a2",
                color: Color::White,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "b2",
                color: Color::Black,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "c2",
                color: Color::White,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "d2",
                color: Color::Black,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "e2",
                color: Color::White,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "f2",
                color: Color::Black,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "g2",
                color: Color::White,
                piece: Piece::WhitePawn,
            },
            Square {
                coord: "h2",
                color: Color::Black,
                piece: Piece::WhitePawn,
            },
        ],
        [
            Square {
                coord: "a1",
                color: Color::Black,
                piece: Piece::WhiteRook,
            },
            Square {
                coord: "b1",
                color: Color::White,
                piece: Piece::WhiteKnight,
            },
            Square {
                coord: "c1",
                color: Color::Black,
                piece: Piece::WhiteBishop,
            },
            Square {
                coord: "d1",
                color: Color::White,
                piece: Piece::WhiteQueen,
            },
            Square {
                coord: "e1",
                color: Color::Black,
                piece: Piece::WhiteKing,
            },
            Square {
                coord: "f1",
                color: Color::White,
                piece: Piece::WhiteBishop,
            },
            Square {
                coord: "g1",
                color: Color::Black,
                piece: Piece::WhiteKnight,
            },
            Square {
                coord: "h1",
                color: Color::White,
                piece: Piece::WhiteRook,
            },
        ],
    ],
};
