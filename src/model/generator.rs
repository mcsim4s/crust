use std::ops::Div;

use crate::model::*;

impl Board {
    pub fn active_color(&self) -> u8 {
        if self.white_is_active {
            WHITE
        } else {
            BLACK
        }
    }

    pub fn gen_moves(&self) -> Vec<Move> {
        let mut buffer: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        for (pos, &piece) in self.squares.iter().enumerate() {
            if piece.is_color(active_color) {
                if piece.is_pawn() {
                    buffer.push(Move {
                        from: pos,
                        to: pos + 8,
                        promote_to: None,
                    });
                }
                if piece.is_king() {
                    let mut king_moves = self.king_moves(pos);
                    dbg!(&king_moves);
                    buffer.append(&mut king_moves);
                }
            }
        }

        buffer
    }

    fn king_moves(&self, king_index: usize) -> Vec<Move> {
        let candidates: [usize; 8] = [
            king_index - 9,
            king_index - 8,
            king_index - 7,
            king_index - 1,
            king_index + 1,
            king_index + 7,
            king_index + 8,
            king_index + 9,
        ];
        candidates
            .into_iter()
            .filter(|&x| x < 64 && x >= 0)
            .map(|x| Move {
                from: king_index,
                to: x,
                promote_to: None,
            })
            .collect()
    }
}
