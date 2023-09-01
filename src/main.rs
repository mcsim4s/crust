mod crust;
mod uci;

use std::io;
use uci::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut engine = crust::Engine {};
    loop {
        io::stdin().read_line(&mut buffer)?;
        let command = Command::parse(&buffer.trim())?;
        engine.execute(command)?;
        buffer.clear();
    }
}
