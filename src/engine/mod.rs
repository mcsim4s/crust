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
            Command::NewGame => (),
            Command::SetPosition {
                position: _,
                moves: _,
            } => (),
            Command::Go(_) => println!("bestmove d7d5"),
        }
        Result::Ok(())
    }

    pub fn new() -> Engine {
        Engine {
            board: Board::from_fen(String::from("")).expect("unexpected starting fen pos"),
        }
    }
}
