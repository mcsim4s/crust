pub mod engine;
pub mod model;
mod uci;
mod util;

use std::io;
use uci::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut engine = engine::Engine::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer == "quit" {
            break;
        }
        let command = Command::parse(&buffer.trim())?;
        engine.execute_uci(command)?;
        buffer.clear();
    }
    Ok(())
}
