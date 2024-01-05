use crate::model::{Board, Move};
use crate::model::pieces::NONE;

impl Move {
    pub fn is_capture(&self, board: &Board) -> bool {
        board.squares[self.to] != NONE || board.en_passant.is_some_and(|sq| sq == self.to)
    }
}