use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use crate::engine::Engine;
use crate::model::{Board, Move};

#[derive(Clone)]
pub struct SearchState {
    pub score: f32,
    pub board: Board,
    pub alpha: SearchResult,
    pub beta: SearchResult,
    pub depth_left: u8,
    pub current_depth: u8,
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

    pub fn with_evaluation_count(self, count: u64) -> SearchResult {
        Self {
            moves: self.moves,
            evaluations: count,
            ..self
        }
    }

    pub fn new(score: i32) -> SearchResult {
        SearchResult {
            score,
            moves: LinkedList::new(),
            evaluations: 0,
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
            alpha: SearchResult::new(i32::MIN + 1),
            beta: SearchResult::new(i32::MAX - 1),
            depth_left: depth,
            current_depth: 0,
        }
    }

    pub fn make_move(&self, mv: &Move) -> SearchState {
        SearchState {
            alpha: self.beta.flip(),
            beta: self.alpha.flip(),
            depth_left: self.depth_left.wrapping_sub(1),
            board: self.board.make_move(mv),
            current_depth: self.current_depth + 1,
            ..*self
        }
    }
}

impl Engine {
    pub fn search(&self) -> Move {
        let mut result = SearchResult::new(0);

        for i in 2..3 {
            result = self.search_req(SearchState::initial(self.board, i));
            println!("info depth {} {}", i, result);
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
        self.debug(&state, format!("A: {}, B: {}, D: {}, ", state.alpha, state.beta, state.depth_left));
        if state.depth_left == 0 {
            return SearchResult {
                moves: LinkedList::new(),
                score: self.board.evaluate(),
                evaluations: 1,
            };
        }
        let mut evaluation_counter = 0u64;
        for mv in state.board.gen_moves(false) {
            self.debug(&state, format!("Making move: {}", mv.to_notation()));
            let mut move_result = self.search_req(state.make_move(&mv)).flip();
            evaluation_counter += move_result.evaluations;
            move_result.moves.push_front(mv);
            if move_result.score >= state.beta.score {
                return state.beta.with_evaluation_count(evaluation_counter);
            }
            if move_result.score > state.alpha.score {
                state.alpha = move_result;
            }
        }

        state.alpha.with_evaluation_count(evaluation_counter)
    }

    fn quiescence(&self, mut state: SearchState) -> SearchResult {
        let eval = state.board.evaluate();
        let mut evaluation_counter = 1u64;
        self.debug(&state, format!("A: {}, B: {}, D: {}, ", state.alpha, state.beta, state.depth_left));
        self.debug(&state, format!("Evaluation: {}", eval));
        if eval >= state.beta.score {
            return state.beta.with_evaluation_count(evaluation_counter);
        }
        if eval > state.alpha.score {
            state.alpha.score = eval;
        }

        for mv in state.board.gen_moves(true) {
            self.debug(&state, format!("Making move: {}", mv.to_notation()));
            let move_result = self.quiescence(state.make_move(&mv)).flip();
            evaluation_counter += move_result.evaluations;
            if move_result.score >= state.beta.score {
                return state.beta.with_evaluation_count(evaluation_counter);
            }
            if move_result.score > state.alpha.score {
                state.alpha = move_result;
            }
        }

        state.alpha.with_evaluation_count(evaluation_counter)
    }

    #[allow(dead_code)]
    fn debug(&self, state: &SearchState, str: String) -> () {
        // for _i in 0..state.current_depth {
        //     print!("\t")
        // }
        // println!("{}", str);
    }
}