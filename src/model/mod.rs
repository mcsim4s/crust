pub mod generator;
pub mod pieces;
mod static_data;
mod tests;

use std::fmt::Debug;

use crate::util::*;
use pieces::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotionKind {
    Rook,
    Knight,
    Bishop,
    Quieen,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: usize,
    to: usize,
    promote_to: Option<PromotionKind>,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub squares: [u8; 64],
    pub white_is_active: bool,
}

impl Move {
    pub fn null() -> Move {
        Move {
            from: 0,
            to: 0,
            promote_to: None,
        }
    }

    pub fn to_notation(&self) -> String {
        let from = index_to_square_notation(self.from).expect("Unable to convert 'from' to notation");
        let to = index_to_square_notation(self.to).expect("Unable to convert 'to' to notation");
        format!("{from}{to}")
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

        Ok(Board { squares, white_is_active })
    }

    pub fn make_move(&self, mv: &Move) -> Board {
        let mut result = self.clone();
        result.squares[mv.to] = result.squares[mv.from];
        result.squares[mv.from] = 0;
        result.white_is_active = !result.white_is_active;
        result
    }

    pub fn make_uci_move(&self, mv: &crate::uci::Move) -> Board {
        let mut result = self.clone();
        result.squares[mv.to] = result.squares[mv.from];
        result.squares[mv.from] = 0;
        result.white_is_active = !result.white_is_active;
        result
    }
}
