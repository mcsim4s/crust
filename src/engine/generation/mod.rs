use crate::engine::generation::static_data::*;
use crate::model::pieces::*;
use crate::model::*;

mod static_data;
mod tests;

impl Board {
    pub fn gen_moves(&self, only_captures: bool) -> Vec<Move> {
        self.gen_pseudo_legal_moves()
            .into_iter()
            .filter(|mv| {
                let inactive_color = self.inactive_color();
                let next_pos = self.make_move(mv);
                let pseudo = next_pos.gen_pseudo_legal_moves();

                if mv.castle {
                    let castle_index = match mv.to {
                        2 => 0,
                        6 => 1,
                        58 => 2,
                        _ => 3,
                    };
                    let castle_squares = CASTLE_SQUARES[castle_index];
                    let castle_pawns = CASTLE_PAWNS[castle_index];
                    let no_attack = !pseudo.iter().any(|next_move| castle_squares.contains(&next_move.to));
                    let no_pawns = !castle_pawns.iter().any(|&square| self.squares[square].is(PAWN, inactive_color));
                    no_attack && no_pawns
                } else {
                    !pseudo.into_iter().any(|next_move| next_pos.squares[next_move.to].is_king())
                }
            })
            .filter(|mv| {
                !only_captures || self.squares[mv.to].is_color(self.inactive_color())
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
            .map(|&x| Move::regular(pos, x))
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
                result.push(Move::regular(pos, to));
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
                    result.push(Move::regular(pos, to));
                }
            }
        }
        if active_color == WHITE && self.castle_white_king && self.squares[61..63] == [NONE, NONE] {
            result.push(Move::castle(60, 62));
        }
        if active_color == WHITE && self.castle_white_queen && self.squares[57..60] == [NONE, NONE, NONE] {
            result.push(Move::castle(60, 58));
        }
        if active_color == BLACK && self.castle_black_king && self.squares[5..7] == [NONE, NONE] {
            result.push(Move::castle(4, 6));
        }
        if active_color == BLACK && self.castle_black_queen && self.squares[1..4] == [NONE, NONE, NONE] {
            result.push(Move::castle(4, 2));
        }

        result
    }

    fn pawn_moves(&self, pos: usize) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let active_color = self.active_color();
        let diff: i8 = if active_color == pieces::WHITE { -8 } else { 8 };
        let regular_move = (pos as i8 + diff) as usize;
        let rank = regular_move / 8;

        if self.squares[regular_move] == pieces::NONE {
            if rank == 0 || rank == 7 {
                result.append(&mut self.gen_promotions(pos, regular_move));
            } else {
                result.push(Move::regular(pos, regular_move));
            }
        }
        let double_move_avaliable = (rank == 2 && active_color == BLACK) || (rank == 5 && active_color == WHITE);
        let double_move = (regular_move as i8 + diff) as usize;
        if double_move_avaliable && result.len() > 0 && self.squares[double_move] == NONE {
            result.push(Move::regular(pos, double_move));
        }
        if pos % 8 != 0 && (self.squares[regular_move - 1].is_color(self.inactive_color()) || self.en_passant == Some(regular_move - 1)) {
            if rank == 0 || rank == 7 {
                result.append(&mut self.gen_promotions(pos, regular_move - 1));
            } else {
                result.push(Move::regular(pos, regular_move - 1));
            }
        }
        if pos % 8 != 7 && (self.squares[regular_move + 1].is_color(self.inactive_color()) || self.en_passant == Some(regular_move + 1)) {
            if rank == 0 || rank == 7 {
                result.append(&mut self.gen_promotions(pos, regular_move + 1));
            } else {
                result.push(Move::regular(pos, regular_move + 1));
            }
        }
        result
    }

    fn gen_promotions(&self, from: usize, to: usize) -> Vec<Move> {
        vec![
            Move::promotion(from, to, QUEEN),
            Move::promotion(from, to, ROOK),
            Move::promotion(from, to, KNIGHT),
            Move::promotion(from, to, BISHOP),
        ]
    }
}
