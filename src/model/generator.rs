use crate::model::*;
use static_data::*;

impl Board {
    pub fn active_color(&self) -> u8 {
        if self.white_is_active {
            WHITE
        } else {
            BLACK
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

        buffer.into_iter().filter(|&x| !self.squares[x.to].is_color(active_color)).collect()
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
        KNIGHT_MOVES[pos]
            .into_iter()
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
                })
            }
        }
        result
    }

    fn king_moves(&self, king_index: usize) -> Vec<Move> {
        let candidates: [Option<usize>; 8] = [
            king_index.checked_sub(9),
            king_index.checked_sub(8),
            king_index.checked_sub(7),
            king_index.checked_sub(1),
            king_index.checked_add(1),
            king_index.checked_add(7),
            king_index.checked_add(8),
            king_index.checked_add(9),
        ];
        candidates
            .into_iter()
            .flatten()
            .filter(|&x| x < 64)
            .map(|x| Move {
                from: king_index,
                to: x,
                promote_to: None,
            })
            .collect()
    }

    fn pawn_moves(&self, pos: usize) -> Vec<Move> {
        let regular_move = if self.active_color() == pieces::WHITE { pos - 8 } else { pos + 8 };
        vec![Move {
            from: pos,
            to: regular_move,
            promote_to: None,
        }]
    }
}
