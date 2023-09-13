mod generation;
mod static_data;
mod tests;

use crate::model::pieces::Piece;
use crate::model::*;
use crate::uci;
use crate::uci::Command;
use rand::RngCore;
use std::io::*;
use std::time::SystemTime;

pub struct Engine {
    pub board: Board,
}

impl Engine {
    pub fn execute_uci(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Uci => println!("uciok"),
            Command::IsReady => println!("readyok"),
            Command::NewGame => self.board = Board::new(),
            Command::SetPosition { position, moves } => {
                match position {
                    crate::uci::Position::Start => self.board = Board::new(),
                    crate::uci::Position::Fen(fen) => self.board = Board::from_fen(fen)?,
                };
                for mv in &moves {
                    self.board = self.board.make_move(&self.uci_move_to_inner_model(&mv));
                }
            }
            Command::Go(_) => {
                let moves = self.board.gen_moves();
                let rnd: usize = (rand::thread_rng().next_u64() % moves.len() as u64) as usize;
                let random_move = moves[rnd].clone();
                let best_move = match moves.into_iter().find(|x| x.castle) {
                    Some(mv) => mv.to_notation(),
                    None => random_move.to_notation(),
                };

                println!("bestmove {best_move}");
            }
            Command::Perft(depth) => {
                self.performance_test_print(depth);
            }
            Command::Display => {
                println!("{}", self.board.to_fen());
            }
        }
        Result::Ok(())
    }

    fn uci_move_to_inner_model(&self, mv: &uci::Move) -> Move {
        let castle = self.board.squares[mv.from].is_king() && mv.from.abs_diff(mv.to) == 2;
        Move {
            from: mv.from,
            to: mv.to,
            promote_to: mv.promote_to,
            castle,
        }
    }

    pub fn new() -> Engine {
        Engine { board: Board::new() }
    }

    pub fn performance_test(&self, depth: u8) -> u64 {
        self.performance_test_inner(depth, false)
    }

    pub fn performance_test_print(&self, depth: u8) -> u64 {
        self.performance_test_inner(depth, true)
    }
    fn performance_test_inner(&self, depth: u8, print: bool) -> u64 {
        let now = SystemTime::now();
        let result = self.performance_test_recursive(&self.board, depth, print);
        let elapsed = now.elapsed().unwrap();
        println!(
            "Perf. depth: {depth}\telapsed: {}ms\tnodes: {result}\tnps: {:.0}",
            elapsed.as_millis(),
            result as f64 / (elapsed.as_millis() as f64 / 1000f64)
        );
        result
    }

    fn performance_test_recursive(&self, board: &Board, depth: u8, print: bool) -> u64 {
        match depth {
            0 => 0,
            1 => {
                let moves = board.gen_moves();
                if print {
                    for mv in &moves {
                        println!("{}: 1", mv.to_notation());
                    }
                }
                moves.len() as u64
            }
            other => {
                let mut result = 0;
                let moves = board.gen_moves();
                for mv in &moves {
                    let acc = self.performance_test_recursive(&board.make_move(mv), other - 1, false);
                    if print {
                        println!("{}: {acc}", mv.to_notation());
                    }
                    result += acc;
                }
                result
            }
        }
    }
}
