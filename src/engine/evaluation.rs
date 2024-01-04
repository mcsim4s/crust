use crate::model::Board;
use crate::model::pieces::*;

impl Board {
    pub fn evaluate(&self) -> f32 {
        let mut result = 0f32;
        for square in self.squares {
            if square.is_color(WHITE) {
                result += 1f32;
            }
            if square.is_color(BLACK) {
                result -= 1f32;
            }
        }

        if self.white_is_active {
            result
        } else {
            -result
        }
    }
}