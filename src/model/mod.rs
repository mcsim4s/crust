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
        let from = Move::index_to_square_notation(self.from)
            .expect("Unable to convert 'from' to notation");
        let to =
            Move::index_to_square_notation(self.to).expect("Unable to convert 'to' to notation");
        format!("{from}{to}")
    }

    pub fn from_notation(mv: &str) -> std::io::Result<Move> {
        let from = Move::square_notation_to_index(&mv[0..2])?;
        let to = Move::square_notation_to_index(&mv[2..4])?;
        Ok(Move {
            from,
            to,
            promote_to: None, //ToDo promotion
        })
    }

    pub fn square_notation_to_index(square: &str) -> std::io::Result<usize> {
        let square: &[u8] = square.as_bytes();
        let file = match square.get(0) {
            Some(file) if *file >= b'a' && *file <= b'h' => *file - b'a',
            Some(other) => {
                return Result::Err(errors::invalid_input(format!(
                    "Unexpected file identifier {other}"
                )))
            }
            None => {
                return Result::Err(errors::invalid_input(format!(
                    "Unexpected empty file identifier"
                )))
            }
        };
        let row: u8 = match square.get(1) {
            Some(row) if row.is_ascii_digit() => row - b'0',
            Some(row) => {
                return Result::Err(errors::invalid_input(format!(
                    "Expected row num but got '{row}'"
                )))
            }
            None => return Result::Err(errors::invalid_input(format!("Unexpected empty row num"))),
        };
        Result::Ok(((8 - row) * 8 + file) as usize)
    }

    pub fn index_to_square_notation(index: usize) -> std::io::Result<String> {
        let index = index as u8;
        if index >= 64 {
            return Result::Err(errors::invalid_input(format!(
                "Square index must be i < 64, but got {index}"
            )));
        }
        let file: char = (b'a' + index % 8) as char;
        let row: u8 = 8 - index / 8;
        Ok(format!("{file}{row}"))
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_notation())
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_notation())
    }
}

impl Board {
    pub fn new() -> Board {
        Board::from_fen(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ))
        .expect("Failed to construct start board")
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
                    let empty_count = usize::try_from(number.to_digit(10).unwrap())
                        .expect("Unable to convert char to usize???");
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
            return Result::Err(errors::invalid_input(format!(
                "Expected space after pieces string"
            )));
        }
        let active_color = match (&mut fen).next() {
            Some('w') => White,
            Some('b') => Black,
            _ => {
                return Result::Err(errors::invalid_input(format!(
                    "Expected active color after fen string"
                )))
            }
        };

        Ok(Board {
            pieces,
            active_color,
        })
    }

    pub fn gen_moves(&self) -> Box<[Move]> {
        let mut buffer = [Move::null(); 218];
        let mut actual_count = 0;
        for (pos, &square) in self.pieces.iter().enumerate() {
            match square {
                Some(piece) if piece.color == self.active_color => match piece.kind {
                    Pawn => {
                        buffer[actual_count] = Move {
                            from: pos,
                            to: pos + 8,
                            promote_to: None,
                        };
                        actual_count += 1;
                    }
                    Rook => (),
                    Knight => (),
                    Bishop => (),
                    Quieen => (),
                    King => (),
                },
                _ => (),
            }
        }

        Box::from(&buffer[..actual_count])
    }

    pub fn make_move(&mut self, mv: &Move) {
        self.pieces[mv.to] = self.pieces[mv.from];
        self.pieces[mv.from] = None;
        match self.active_color {
            White => self.active_color = Black,
            Black => self.active_color = White,
        };
    }
}
