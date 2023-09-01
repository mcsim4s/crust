use super::uci::Command;
use std::io::*;

pub struct Engine {}

impl Engine {
    pub fn execute(&mut self, command: Command) -> Result<()> {
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
}
