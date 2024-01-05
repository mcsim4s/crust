use crate::model::Board;
use crate::model::pieces::*;

// NONE, KING, PAWN, KNIGHT. BISHOP, ROOK, QUEEN
static PIECE_VALUES: [i32; 7] = [0, 0, 100, 300, 310, 600, 900];

impl Board {
    pub fn evaluate(&self) -> i32 {
        if self.gen_moves(false).is_empty() {
            return i32::MIN + 100;
        }
        let mut result = 0;
        for square in self.squares {
            let index = square.without_color() as usize;
            let mut value = PIECE_VALUES[index];
            if square.is_color(BLACK) {
                value *= -1;
            }
            result += value;
        }

        if self.white_is_active {
            result
        } else {
            -result
        }
    }
}