use crate::model::*;

impl Board {
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

    fn king_moves(&self, king_index: usize) -> Box<[Move]> {
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
            .filter(|&x| match self.pieces[x] {
                Some(target_piece) => target_piece.color != self.active_color,
                None => true,
            })
            .map(|x| Move {
                from: king_index,
                to: x,
                promote_to: None,
            })
            .collect()
    }
}
