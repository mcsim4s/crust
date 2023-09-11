use rand::RngCore;

use crate::model::*;
use crate::uci::Command;
use std::io::*;
use std::time::SystemTime;

pub struct Engine {
    board: Board,
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
                    self.board = self.board.make_move(&mv.to_inner_model());
                }
            }
            Command::Go(_) => {
                let moves = self.board.gen_moves();
                let rnd: usize = (rand::thread_rng().next_u64() % moves.len() as u64) as usize;
                let best_move = moves[rnd].to_notation();
                println!("bestmove {best_move}");
            }
        }
        Result::Ok(())
    }

    pub fn new() -> Engine {
        Engine { board: Board::new() }
    }

    pub fn performance_test(&self, depth: u8) -> u64 {
        let now = SystemTime::now();
        let result = self.performance_test_inner(&self.board, depth);
        println!(
            "Perf test for depth {depth} completed. Elapsed time {}ms. Nodes searched: {result}",
            now.elapsed().unwrap().as_millis()
        );
        result
    }

    fn performance_test_inner(&self, board: &Board, depth: u8) -> u64 {
        match depth {
            0 => 0,
            1 => board.gen_moves().len() as u64,
            other => {
                let mut result = 0;
                let moves = board.gen_moves();
                for mv in &moves {
                    result += self.performance_test_inner(&board.make_move(mv), other - 1);
                }
                result
            }
        }
    }
}
