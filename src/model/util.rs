pub mod errors {
    use std::io::ErrorKind;

    pub fn new(kind: ErrorKind, msg: String) -> std::io::Error {
        std::io::Error::new(kind, msg)
    }

    pub fn invalid_input(msg: String) -> std::io::Error {
        new(ErrorKind::InvalidInput, msg)
    }
}

pub fn square_notation_to_index(square: &str) -> std::io::Result<usize> {
    let square: &[u8] = square.as_bytes();
    let file = match square.get(0) {
        Some(file) if *file >= b'a' && *file <= b'h' => *file - b'a',
        Some(other) => return Result::Err(errors::invalid_input(format!("Unexpected file identifier {other}"))),
        None => return Result::Err(errors::invalid_input(format!("Unexpected empty file identifier"))),
    };
    let rank: u8 = match square.get(1) {
        Some(row) if row.is_ascii_digit() => row - b'0',
        Some(row) => return Result::Err(errors::invalid_input(format!("Expected rank num but got '{row}'"))),
        None => return Result::Err(errors::invalid_input(format!("Unexpected empty rank num"))),
    };
    Result::Ok(((8 - rank) * 8 + file) as usize)
}

pub fn index_to_square_notation(index: usize) -> std::io::Result<String> {
    let index = index as u8;
    if index >= 64 {
        return Result::Err(errors::invalid_input(format!("Square index must be i < 64, but got {index}")));
    }
    let file: char = (b'a' + index % 8) as char;
    let rank: u8 = 8 - index / 8;
    Ok(format!("{file}{rank}"))
}
