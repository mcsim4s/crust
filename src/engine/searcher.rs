use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use crate::engine::Engine;
use crate::model::{Board, Move};

#[derive(Clone)]
pub struct SearchState {
    pub score: f32,
    pub board: Board,
    pub best_move: SearchResult,
    pub worst_move: SearchResult,
    pub depth_left: u8,
    pub current_depth: u8,
}

#[derive(Clone)]
pub struct SearchResult {
    pub moves: LinkedList<Move>,
    pub score: f32,
}

impl SearchResult {
    pub fn flip(&self) -> Self {
        Self {
            score: -self.score,
            moves: self.moves.clone(),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(format!("score cp {} pv ", self.score).as_str());
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
            best_move: SearchResult {
                score: f32::MIN,
                moves: LinkedList::new(),
            },
            worst_move: SearchResult {
                score: f32::MAX,
                moves: LinkedList::new(),
            },
            depth_left: depth,
            current_depth: 0,
        }
    }
}

impl Engine {
    pub fn search(&self) -> Move {
        let mut result = SearchResult {
            moves: LinkedList::new(),
            score: 0f32,
        };

        for i in 1..5 {
            result = self.search_req(SearchState::initial(self.board, i));
            println!("info depth {} {}", i, result);
        }

        match result.moves.pop_front() {
            None => {
                self.board.gen_moves().first().unwrap().clone()
            }
            Some(move_found) => {
                move_found
            }
        }
    }


    pub fn search_req(&self, mut state: SearchState) -> SearchResult {
        // self.debug(&state, format!("A: {}, B: {}, D: {}, ", state.best_move, state.worst_move, state.depth_left));
        if state.depth_left == 0 {
            // self.debug(&state, format!("Evaluation: {}", state.board.evaluate()));
            return SearchResult {
                score: state.board.evaluate(),
                moves: LinkedList::new(),
            };
        }
        for mv in state.board.gen_moves() {
            // self.debug(&state, format!("Making move: {}", mv.to_notation()));
            let new_board = state.board.make_move(&mv);
            let mut move_result = self.search_req(SearchState {
                best_move: state.worst_move.flip(),
                worst_move: state.best_move.flip(),
                depth_left: state.depth_left - 1,
                board: new_board,
                current_depth: state.current_depth + 1,
                ..state
            });
            move_result.moves.push_front(mv);
            let move_eval = -move_result.score;
            if move_eval >= state.worst_move.score {
                return state.worst_move;
            }
            if move_eval > state.best_move.score {
                state.best_move = move_result.flip();
            }
        }

        state.best_move
    }

    #[allow(dead_code)]
    fn debug(&self, state: &SearchState, str: String) -> () {
        for _i in 0..state.current_depth {
            print!("\t")
        }
        println!("{}", str);
    }
}