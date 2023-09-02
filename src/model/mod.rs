mod tests;

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

pub struct Board {
    pub pieces: [Option<Piece>; 64],
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

impl Board {
    pub fn new() -> Board {
        Board::from_fen(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ))
        .expect("Failed to construct start board")
    }

    pub fn from_fen(fen: String) -> std::io::Result<Board> {
        let mut pieces: [Option<Piece>; 64] = [None; 64];
        let mut current = 0;
        for symbol in fen.chars() {
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
        Ok(Board { pieces })
    }
}
