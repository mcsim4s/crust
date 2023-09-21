mod engine;
mod model;
mod uci;

use std::io;
use uci::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut engine = engine::Engine::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer == "quit\n" {
            break;
        }
        match Command::parse(&buffer.trim()) {
            Ok(command) => {
                engine.execute_uci(command)?;
                buffer.clear();
            }
            Err(err) => println!("{}", err)
        }
        buffer.clear();
    }
    Ok(())
}
