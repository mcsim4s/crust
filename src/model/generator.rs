use crate::{model::*, util};
use static_data::*;

impl Board {
    pub fn active_color(&self) -> u8 {
        if self.white_is_active {
            WHITE
        } else {
            BLACK
        }
    }
    pub fn inactive_color(&self) -> u8 {
        if self.white_is_active {
            BLACK
        } else {
            WHITE
        }
    }

    pub fn gen_moves(&self) -> Vec<Move> {
        self.gen_pseudo_legal_moves()
            .into_iter()
            .filter(|mv| {
                let next_pos = self.make_move(mv);
                let pseudo = next_pos.gen_pseudo_legal_moves();
                !pseudo.into_iter().any(|next_move| next_pos.squares[next_move.to].is_king())
            })
            .collect()
    }

    fn gen_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut buffer: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        for (pos, &piece) in self.squares.iter().enumerate() {
            if piece.is_color(active_color) {
                buffer.append(&mut self.gen_piece_moves(piece, pos));
            }
        }

        buffer
    }

    fn gen_piece_moves(&self, piece: u8, pos: usize) -> Vec<Move> {
        if piece.is_pawn() {
            self.pawn_moves(pos)
        } else if piece.is_king() {
            self.king_moves(pos)
        } else if piece.is_bishop() {
            self.sliding_moves(pos, 4..8)
        } else if piece.is_rook() {
            self.sliding_moves(pos, 0..4)
        } else if piece.is_queen() {
            self.sliding_moves(pos, 0..8)
        } else if piece.is_knight() {
            self.knight_moves(pos)
        } else {
            panic!("Unknonw piece type to gen moves")
        }
    }

    fn knight_moves(&self, pos: usize) -> Vec<Move> {
        let active_color = self.active_color();
        KNIGHT_MOVES[pos]
            .into_iter()
            .filter(|&&x| !self.squares[x as usize].is_color(active_color))
            .map(|&x| Move {
                from: pos,
                to: x,
                promote_to: None,
            })
            .collect()
    }

    fn sliding_moves(&self, pos: usize, directions: std::ops::Range<usize>) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        let inactive_color = self.inactive_color();
        for direction in directions {
            for distance in 1..EDGE_DISTANCE[pos][direction] + 1 {
                let to = (pos as i8 + DIRECTIONS[direction] * distance) as usize;
                if self.squares[to].is_color(active_color) {
                    break;
                }
                result.push(Move {
                    from: pos,
                    to,
                    promote_to: None,
                });
                if self.squares[to].is_color(inactive_color) {
                    break;
                }
            }
        }
        result
    }

    fn king_moves(&self, pos: usize) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        for direction in 0..8 {
            if EDGE_DISTANCE[pos][direction] > 0 {
                let to = (pos as i8 + DIRECTIONS[direction]) as usize;
                if !self.squares[to].is_color(active_color) {
                    result.push(Move {
                        from: pos,
                        to,
                        promote_to: None,
                    });
                }
            }
        }
        result
    }

    fn pawn_moves(&self, pos: usize) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        let diff: i8 = if active_color == pieces::WHITE { -8 } else { 8 };
        let regular_move = (pos as i8 + diff) as usize;
        if self.squares[regular_move] == pieces::NONE {
            result.push(Move {
                from: pos,
                to: regular_move,
                promote_to: None,
            });
        }
        if pos % 8 != 0 && self.squares[regular_move - 1].is_color(self.inactive_color()) {
            result.push(Move {
                from: pos,
                to: regular_move - 1,
                promote_to: None,
            });
        }
        if pos % 8 != 7 && self.squares[regular_move + 1].is_color(self.inactive_color()) {
            result.push(Move {
                from: pos,
                to: regular_move + 1,
                promote_to: None,
            });
        }
        let rank = pos / 8;
        let double_move_avaliable = (rank == 1 && active_color == BLACK) || (rank == 6 && active_color == WHITE);
        let double_move = (regular_move as i8 + diff) as usize;
        if double_move_avaliable && result.len() > 0 && self.squares[double_move] == NONE {
            result.push(Move {
                from: pos,
                to: double_move,
                promote_to: None,
            });
        }
        result
    }
}
