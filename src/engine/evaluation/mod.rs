mod move_extensions;

use crate::model::Board;
use crate::model::pieces::*;


const MATE_SCORE: i32 = i32::MIN + 10;
// NONE, KING, PAWN, KNIGHT. BISHOP, ROOK, QUEEN
static PIECE_VALUES: [i32; 7] = [0, 0, 100, 300, 310, 600, 900];


impl Board {
    pub fn evaluate(&self, depth: u32) -> i32 {
        if self.gen_moves(false).is_empty() {
            return MATE_SCORE.wrapping_add(depth as i32);
        }
        let mut result = 0;
        for piece in self.squares {
            result += Board::value(piece);
        }

        if self.white_is_active {
            result
        } else {
            -result
        }
    }

    fn value(piece: u8) -> i32 {
        let index = piece.without_color() as usize;
        let mut value = PIECE_VALUES[index];
        if piece.is_color(BLACK) {
            value *= -1;
        }
        value
    }
}