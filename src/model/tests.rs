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
}
