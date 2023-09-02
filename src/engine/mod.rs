use crate::model::*;
use crate::uci::Command;
use std::io::*;

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
                    self.board.make_move(mv);
                }
            }
            Command::Go(_) => {
                let moves = self.board.gen_moves();
                let best_move = moves[0];
                println!("bestmove {best_move}");
            }
        }
        Result::Ok(())
    }

    pub fn new() -> Engine {
        Engine {
            board: Board::new(),
        }
    }
}
