pub mod errors {
    use std::io::ErrorKind;

    pub fn new(kind: ErrorKind, msg: String) -> std::io::Error {
        std::io::Error::new(kind, msg)
    }

    pub fn invalid_input(msg: String) -> std::io::Error {
        new(ErrorKind::InvalidInput, msg)
    }
}
