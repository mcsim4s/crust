pub mod generator;
pub mod pieces;
mod static_data;
mod tests;

use std::fmt::Debug;

use crate::util::*;
use pieces::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub promote_to: Option<u8>,
    pub castle: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub squares: [u8; 64],
    pub white_is_active: bool,
    pub castle_white_king: bool,
    pub castle_white_queen: bool,
    pub castle_black_king: bool,
    pub castle_black_queen: bool,
}

impl Move {
    pub fn null() -> Move {
        Move::regular(0, 0)
    }

    pub fn to_notation(&self) -> String {
        let from = index_to_square_notation(self.from).expect("Unable to convert 'from' to notation");
        let to = index_to_square_notation(self.to).expect("Unable to convert 'to' to notation");
        let promotion = match self.promote_to {
            Some(QUEEN) => "q",
            Some(ROOK) => "r",
            Some(KNIGHT) => "k",
            Some(BISHOP) => "b",
            _ => "",
        };
        format!("{from}{to}{promotion}")
    }

    pub fn regular(from: usize, to: usize) -> Move {
        Move {
            from,
            to,
            promote_to: None,
            castle: false,
        }
    }

    pub fn castle(from: usize, to: usize) -> Move {
        Move {
            from,
            to,
            promote_to: None,
            castle: true,
        }
    }

    pub fn promotion(from: usize, to: usize, piece: u8) -> Move {
        Move {
            from,
            to,
            promote_to: Some(piece),
            castle: false,
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_notation().as_str())
    }
}

impl Board {
    pub fn new() -> Board {
        Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")).expect("Failed to construct start board")
    }

    pub fn from_fen(fen: String) -> std::io::Result<Board> {
        let mut fen = fen.chars();
        let mut squares: [u8; 64] = [0; 64];
        let mut current = 0;
        for symbol in &mut fen {
            match symbol {
                'r' => {
                    squares[current] = rook(BLACK);
                    current += 1;
                }
                'n' => {
                    squares[current] = knight(BLACK);
                    current += 1;
                }
                'b' => {
                    squares[current] = bishop(BLACK);
                    current += 1;
                }
                'q' => {
                    squares[current] = quieen(BLACK);
                    current += 1;
                }
                'k' => {
                    squares[current] = king(BLACK);
                    current += 1;
                }
                'p' => {
                    squares[current] = pawn(BLACK);
                    current += 1;
                }
                'R' => {
                    squares[current] = rook(WHITE);
                    current += 1;
                }
                'N' => {
                    squares[current] = knight(WHITE);
                    current += 1;
                }
                'B' => {
                    squares[current] = bishop(WHITE);
                    current += 1;
                }
                'Q' => {
                    squares[current] = quieen(WHITE);
                    current += 1;
                }
                'K' => {
                    squares[current] = king(WHITE);
                    current += 1;
                }
                'P' => {
                    squares[current] = pawn(WHITE);
                    current += 1;
                }
                '/' => (),
                number if number.is_digit(10) => {
                    let empty_count = usize::try_from(number.to_digit(10).unwrap()).expect("Unable to convert char to usize???");
                    current += empty_count;
                }
                _parse_start_fenother => todo!("Error"),
            }
            if current >= 64 {
                break;
            }
        }

        let space = (&mut fen).next();
        if space != Some(' ') {
            return Result::Err(errors::invalid_input(format!("Expected space after pieces string")));
        }
        let white_is_active = match (&mut fen).next() {
            Some('w') => true,
            Some('b') => false,
            _ => return Result::Err(errors::invalid_input(format!("Expected active color after fen string"))),
        };
        let space = (&mut fen).next();
        if space != Some(' ') {
            return Result::Err(errors::invalid_input(format!("Expected space after pieces string")));
        }
        let mut castle_white_king = false;
        let mut castle_white_queen = false;
        let mut castle_black_king = false;
        let mut castle_black_queen = false;
        #[allow(irrefutable_let_patterns)]
        while let next = (&mut fen).next() {
            match next {
                Some('K') => castle_white_king = true,
                Some('k') => castle_black_king = true,
                Some('Q') => castle_white_queen = true,
                Some('q') => castle_black_queen = true,
                Some(' ') => break,
                _ => return Result::Err(errors::invalid_input(format!("Expected castle state in fen string"))),
            }
        }

        Ok(Board {
            squares,
            white_is_active,
            castle_white_king,
            castle_white_queen,
            castle_black_king,
            castle_black_queen,
        })
    }

    pub fn make_move(&self, mv: &Move) -> Board {
        let mut result = self.clone();

        // Disable castling
        if mv.from == 60 {
            result.castle_white_king = false;
            result.castle_white_queen = false;
        }
        if mv.from == 63 || mv.to == 63 {
            result.castle_white_king = false;
        }
        if mv.from == 56 || mv.to == 56 {
            result.castle_white_queen = false;
        }
        if mv.from == 4 {
            result.castle_black_king = false;
            result.castle_black_queen = false;
        }
        if mv.from == 7 || mv.to == 7 {
            result.castle_black_king = false;
        }
        if mv.from == 0 || mv.to == 0 {
            result.castle_black_queen = false;
        }

        result.squares[mv.to] = match mv.promote_to {
            Some(piece) => pieces::new(piece, self.active_color()),
            None => result.squares[mv.from],
        };
        result.squares[mv.from] = 0;
        if mv.castle {
            match mv.to {
                62 => {
                    result.squares[61] = result.squares[63];
                    result.squares[63] = NONE;
                }
                58 => {
                    result.squares[59] = result.squares[56];
                    result.squares[56] = NONE;
                }
                6 => {
                    result.squares[5] = result.squares[7];
                    result.squares[7] = NONE;
                }
                2 => {
                    result.squares[3] = result.squares[0];
                    result.squares[0] = NONE;
                }
                _ => (),
            }
        }
        result.white_is_active = !result.white_is_active;
        result
    }
}
