mod tests;

use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use std::time::{Duration, SystemTime};
use crate::engine::Engine;
use crate::model::{Board, Move};

#[derive(Clone, Copy)]
pub struct SearchState {
    pub score: f32,
    pub board: Board,
    pub alpha: i32,
    pub beta: i32,
    pub depth_left: i8,
    pub current_depth: u32,
    pub start: SystemTime,
}

#[derive(Clone)]
pub struct SearchResult {
    pub moves: LinkedList<Move>,
    pub score: i32,
    pub evaluations: u64,
}

impl SearchResult {
    pub fn flip(&self) -> Self {
        Self {
            score: self.score.neg(),
            moves: self.moves.clone(),
            ..*self
        }
    }

    pub fn new(score: i32, evaluations: u64) -> SearchResult {
        SearchResult {
            score,
            moves: LinkedList::new(),
            evaluations,
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(format!("nodes {} score cp {} pv ", self.evaluations, self.score).as_str());
        for mv in self.moves.iter() {
            result.push_str(mv.to_notation().as_str());
            result.push_str(" ");
        }

        write!(f, "{}", result)
    }
}

impl SearchState {
    pub fn initial(board: Board, depth: u8) -> SearchState {
        SearchState {
            score: 0f32,
            board,
            alpha: i32::MIN + 1,
            beta: i32::MAX - 1,
            depth_left: depth as i8,
            current_depth: 0,
            start: SystemTime::now(),
        }
    }

    pub fn make_move(&self, mv: &Move) -> SearchState {
        SearchState {
            alpha: self.beta.neg(),
            beta: self.alpha.neg(),
            depth_left: self.depth_left - 1,
            board: self.board.make_move(mv),
            current_depth: self.current_depth + 1,
            ..*self
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed().unwrap().max(Duration::from_millis(1))
    }
}

impl Engine {
    pub fn search(&self) -> Move {
        let mut result = SearchResult::new(0, 0);

        for i in 1..4 {
            let state = SearchState::initial(self.board, i);
            result = self.search_req(state);
            println!("info depth {} {} time {} nps {:.0}", i, result, state.elapsed().as_millis(), result.evaluations as f64 / state.elapsed().as_secs_f64());
        }

        match result.moves.pop_front() {
            None => {
                self.board.gen_moves(false).first().unwrap().clone()
            }
            Some(move_found) => {
                move_found
            }
        }
    }


    pub fn search_req(&self, mut state: SearchState) -> SearchResult {
        let moves = state.board.order(&state.board.gen_moves(false));
        if state.depth_left == 0 || moves.is_empty() {
            return self.quiescence(state);
        }
        let mut evaluation_counter = 0u64;
        let mut best_line: LinkedList<Move> = LinkedList::new();
        for mv in moves {
            self.debug(&state, format!("A: {}, B: {}, D: {}, ", state.alpha, state.beta, state.depth_left));
            self.debug(&state, format!("Making move: {}", mv.to_notation()));
            let move_result = self.search_req(state.make_move(&mv)).flip();
            evaluation_counter += move_result.evaluations;
            if move_result.score >= state.beta {
                return SearchResult::new(state.beta, evaluation_counter);
            }
            if move_result.score > state.alpha {
                state.alpha = move_result.score;
                best_line = move_result.moves;
                best_line.push_front(mv)
            }
        }

        SearchResult {
            score: state.alpha,
            moves: best_line,
            evaluations: evaluation_counter,
        }
    }

    fn quiescence(&self, mut state: SearchState) -> SearchResult {
        let eval = state.board.evaluate(state.current_depth);
        let mut evaluation_counter = 1u64;
        self.debug(&state, format!("A: {}, B: {}, D: {}, ", state.alpha, state.beta, state.depth_left));
        self.debug(&state, format!("Evaluation: {}", eval));
        if eval >= state.beta {
            return SearchResult::new(state.beta, evaluation_counter);
        }
        state.alpha = state.alpha.max(eval);

        for mv in state.board.order(&state.board.gen_moves(true)) {
            self.debug(&state, format!("Making move: {}", mv.to_notation()));
            let move_result = self.quiescence(state.make_move(&mv)).flip();
            evaluation_counter += move_result.evaluations;
            if move_result.score >= state.beta {
                return SearchResult::new(state.beta, evaluation_counter);
            }
            state.alpha = state.alpha.max(move_result.score);
        }

        return SearchResult::new(state.alpha, evaluation_counter);
    }

    #[allow(dead_code)]
    fn debug(&self, state: &SearchState, str: String) -> () {
        if state.current_depth > 0 || !str.is_empty() {
            // for _i in 0..state.current_depth {
            //     print!("\t")
            // }
            // println!("{}", str);
        }
    }
}