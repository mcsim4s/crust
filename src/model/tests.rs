use crate::model::*;
use PieceColor::*;
use PieceKind::*;

#[test]
fn construct_start_board() {
    let start_board = Board::new();
    assert_eq!(Some(Piece::new(Rook, Black)), start_board.pieces[0]);
    assert_eq!(Some(Piece::new(Knight, Black)), start_board.pieces[1]);
    assert_eq!(Some(Piece::new(Bishop, Black)), start_board.pieces[2]);
    assert_eq!(Some(Piece::new(Quieen, Black)), start_board.pieces[3]);
    assert_eq!(Some(Piece::new(King, Black)), start_board.pieces[4]);
    assert_eq!(Some(Piece::new(Bishop, Black)), start_board.pieces[5]);
    assert_eq!(Some(Piece::new(Knight, Black)), start_board.pieces[6]);
    assert_eq!(Some(Piece::new(Rook, Black)), start_board.pieces[7]);
    for i in 8..15 {
        assert_eq!(Some(Piece::new(Pawn, Black)), start_board.pieces[i]);
    }
    for i in 16..47 {
        assert_eq!(None, start_board.pieces[i]);
    }
    for i in 48..55 {
        assert_eq!(Some(Piece::new(Pawn, White)), start_board.pieces[i]);
    }
    assert_eq!(Some(Piece::new(Rook, White)), start_board.pieces[56]);
    assert_eq!(Some(Piece::new(Knight, White)), start_board.pieces[57]);
    assert_eq!(Some(Piece::new(Bishop, White)), start_board.pieces[58]);
    assert_eq!(Some(Piece::new(Quieen, White)), start_board.pieces[59]);
    assert_eq!(Some(Piece::new(King, White)), start_board.pieces[60]);
    assert_eq!(Some(Piece::new(Bishop, White)), start_board.pieces[61]);
    assert_eq!(Some(Piece::new(Knight, White)), start_board.pieces[62]);
    assert_eq!(Some(Piece::new(Rook, White)), start_board.pieces[63]);

    assert_eq!(White, start_board.active_color);
}

#[test]
fn notation_to_index() {
    assert_eq!(0, Move::square_notation_to_index("a8").unwrap());
    assert_eq!(1, Move::square_notation_to_index("b8").unwrap());
    assert_eq!(2, Move::square_notation_to_index("c8").unwrap());
    assert_eq!(3, Move::square_notation_to_index("d8").unwrap());
    assert_eq!(4, Move::square_notation_to_index("e8").unwrap());
    assert_eq!(5, Move::square_notation_to_index("f8").unwrap());
    assert_eq!(6, Move::square_notation_to_index("g8").unwrap());
    assert_eq!(7, Move::square_notation_to_index("h8").unwrap());

    assert_eq!(8, Move::square_notation_to_index("a7").unwrap());
    assert_eq!(9, Move::square_notation_to_index("b7").unwrap());
    assert_eq!(10, Move::square_notation_to_index("c7").unwrap());
    assert_eq!(11, Move::square_notation_to_index("d7").unwrap());
    assert_eq!(12, Move::square_notation_to_index("e7").unwrap());
    assert_eq!(13, Move::square_notation_to_index("f7").unwrap());
    assert_eq!(14, Move::square_notation_to_index("g7").unwrap());
    assert_eq!(15, Move::square_notation_to_index("h7").unwrap());
}
