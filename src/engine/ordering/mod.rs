use std::cmp::Ordering;
use crate::model::{Board, Move};

impl Board {
    pub fn order(&self, moves: &Vec<Move>) -> Vec<Move> {
        let mut result = moves.clone();
        result.sort_by(|left, right| {
            if left.is_capture(self) {
                if right.is_capture(self) {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                if !right.is_capture(self) {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
        });
        result.to_vec()
    }
}