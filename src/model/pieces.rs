pub const NONE: u8 = 0;
pub const KING: u8 = 1;
pub const PAWN: u8 = 2;
pub const KNIGHT: u8 = 3;
pub const BISHOP: u8 = 4;
pub const ROOK: u8 = 5;
pub const QUEEN: u8 = 6;

pub const WHITE: u8 = 8;
pub const BLACK: u8 = 16;

pub trait Piece {
    fn is_color(&self, color: u8) -> bool;
    fn is_piece(&self, kind: u8) -> bool;
    fn notation(&self) -> char;
    fn without_color(&self) -> u8;

    fn is(&self, kind: u8, color: u8) -> bool {
        self.is_color(color) && self.is_piece(kind)
    }

    fn is_king(&self) -> bool {
        self.is_piece(KING)
    }
    fn is_pawn(&self) -> bool {
        self.is_piece(PAWN)
    }
    fn is_knight(&self) -> bool {
        self.is_piece(KNIGHT)
    }
    fn is_bishop(&self) -> bool {
        self.is_piece(BISHOP)
    }
    fn is_rook(&self) -> bool {
        self.is_piece(ROOK)
    }
    fn is_queen(&self) -> bool {
        self.is_piece(QUEEN)
    }
}

impl Piece for u8 {
    fn is_color(&self, color: u8) -> bool {
        (self & color) > 0
    }

    fn is_piece(&self, piece: u8) -> bool {
        (self & 0b00000111) == piece
    }

    fn notation(&self) -> char {
        let mut result = '-';
        if self.is_king() {
            result = 'k';
        }
        if self.is_queen() {
            result = 'q';
        }
        if self.is_knight() {
            result = 'n';
        }
        if self.is_rook() {
            result = 'r';
        }
        if self.is_pawn() {
            result = 'p';
        }
        if self.is_bishop() {
            result = 'b';
        }
        if self.is_color(WHITE) {
            result.to_ascii_uppercase()
        } else {
            result
        }
    }

    fn without_color(&self) -> u8 {
        self & 0b00000111
    }
}

pub fn new(kind: u8, color: u8) -> u8 {
    kind | color
}

pub fn pawn(color: u8) -> u8 {
    new(PAWN, color)
}

pub fn rook(color: u8) -> u8 {
    new(ROOK, color)
}

pub fn knight(color: u8) -> u8 {
    new(KNIGHT, color)
}

pub fn bishop(color: u8) -> u8 {
    new(BISHOP, color)
}

pub fn quieen(color: u8) -> u8 {
    new(QUEEN, color)
}

pub fn king(color: u8) -> u8 {
    new(KING, color)
}
