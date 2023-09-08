mod generator;
mod tests;

use std::fmt::Display;

use crate::util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Quieen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: usize,
    to: usize,
    promote_to: Option<PieceKind>,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub pieces: [Option<Piece>; 64],
    pub active_color: PieceColor,
}

use PieceColor::*;
use PieceKind::*;

impl Piece {
    fn new(kind: PieceKind, color: PieceColor) -> Piece {
        Piece { kind, color }
    }

    pub fn pawn(color: PieceColor) -> Piece {
        Piece::new(Pawn, color)
    }
    pub fn rook(color: PieceColor) -> Piece {
        Piece::new(Rook, color)
    }
    pub fn knight(color: PieceColor) -> Piece {
        Piece::new(Knight, color)
    }
    pub fn bishop(color: PieceColor) -> Piece {
        Piece::new(Bishop, color)
    }
    pub fn quieen(color: PieceColor) -> Piece {
        Piece::new(Quieen, color)
    }
    pub fn king(color: PieceColor) -> Piece {
        Piece::new(King, color)
    }
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

impl Board {
    pub fn new() -> Board {
        Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")).expect("Failed to construct start board")
    }

    pub fn from_fen(fen: String) -> std::io::Result<Board> {
        let mut fen = fen.chars();
        let mut pieces: [Option<Piece>; 64] = [None; 64];
        let mut current = 0;
        for symbol in &mut fen {
            match symbol {
                'r' => {
                    pieces[current] = Some(Piece::rook(Black));
                    current += 1;
                }
                'n' => {
                    pieces[current] = Some(Piece::knight(Black));
                    current += 1;
                }
                'b' => {
                    pieces[current] = Some(Piece::bishop(Black));
                    current += 1;
                }
                'q' => {
                    pieces[current] = Some(Piece::quieen(Black));
                    current += 1;
                }
                'k' => {
                    pieces[current] = Some(Piece::king(Black));
                    current += 1;
                }
                'p' => {
                    pieces[current] = Some(Piece::pawn(Black));
                    current += 1;
                }
                'R' => {
                    pieces[current] = Some(Piece::rook(White));
                    current += 1;
                }
                'N' => {
                    pieces[current] = Some(Piece::knight(White));
                    current += 1;
                }
                'B' => {
                    pieces[current] = Some(Piece::bishop(White));
                    current += 1;
                }
                'Q' => {
                    pieces[current] = Some(Piece::quieen(White));
                    current += 1;
                }
                'K' => {
                    pieces[current] = Some(Piece::king(White));
                    current += 1;
                }
                'P' => {
                    pieces[current] = Some(Piece::pawn(White));
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
        let active_color = match (&mut fen).next() {
            Some('w') => White,
            Some('b') => Black,
            _ => return Result::Err(errors::invalid_input(format!("Expected active color after fen string"))),
        };

        Ok(Board { pieces, active_color })
    }

    pub fn make_uci_move(&mut self, mv: &crate::uci::Move) -> Board {
        let mut result = self.clone();
        result.pieces[mv.to] = result.pieces[mv.from];
        result.pieces[mv.from] = None;
        match result.active_color {
            White => result.active_color = Black,
            Black => result.active_color = White,
        };
        result
    }
}
