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

    assert_eq!(16, Move::square_notation_to_index("a6").unwrap());
    assert_eq!(17, Move::square_notation_to_index("b6").unwrap());
    assert_eq!(18, Move::square_notation_to_index("c6").unwrap());
    assert_eq!(19, Move::square_notation_to_index("d6").unwrap());
    assert_eq!(20, Move::square_notation_to_index("e6").unwrap());
    assert_eq!(21, Move::square_notation_to_index("f6").unwrap());
    assert_eq!(22, Move::square_notation_to_index("g6").unwrap());
    assert_eq!(23, Move::square_notation_to_index("h6").unwrap());

    assert_eq!(24, Move::square_notation_to_index("a5").unwrap());
    assert_eq!(25, Move::square_notation_to_index("b5").unwrap());
    assert_eq!(26, Move::square_notation_to_index("c5").unwrap());
    assert_eq!(27, Move::square_notation_to_index("d5").unwrap());
    assert_eq!(28, Move::square_notation_to_index("e5").unwrap());
    assert_eq!(29, Move::square_notation_to_index("f5").unwrap());
    assert_eq!(30, Move::square_notation_to_index("g5").unwrap());
    assert_eq!(31, Move::square_notation_to_index("h5").unwrap());

    assert_eq!(32, Move::square_notation_to_index("a4").unwrap());
    assert_eq!(33, Move::square_notation_to_index("b4").unwrap());
    assert_eq!(34, Move::square_notation_to_index("c4").unwrap());
    assert_eq!(35, Move::square_notation_to_index("d4").unwrap());
    assert_eq!(36, Move::square_notation_to_index("e4").unwrap());
    assert_eq!(37, Move::square_notation_to_index("f4").unwrap());
    assert_eq!(38, Move::square_notation_to_index("g4").unwrap());
    assert_eq!(39, Move::square_notation_to_index("h4").unwrap());

    assert_eq!(40, Move::square_notation_to_index("a3").unwrap());
    assert_eq!(41, Move::square_notation_to_index("b3").unwrap());
    assert_eq!(42, Move::square_notation_to_index("c3").unwrap());
    assert_eq!(43, Move::square_notation_to_index("d3").unwrap());
    assert_eq!(44, Move::square_notation_to_index("e3").unwrap());
    assert_eq!(45, Move::square_notation_to_index("f3").unwrap());
    assert_eq!(46, Move::square_notation_to_index("g3").unwrap());
    assert_eq!(47, Move::square_notation_to_index("h3").unwrap());

    assert_eq!(48, Move::square_notation_to_index("a2").unwrap());
    assert_eq!(49, Move::square_notation_to_index("b2").unwrap());
    assert_eq!(50, Move::square_notation_to_index("c2").unwrap());
    assert_eq!(51, Move::square_notation_to_index("d2").unwrap());
    assert_eq!(52, Move::square_notation_to_index("e2").unwrap());
    assert_eq!(53, Move::square_notation_to_index("f2").unwrap());
    assert_eq!(54, Move::square_notation_to_index("g2").unwrap());
    assert_eq!(55, Move::square_notation_to_index("h2").unwrap());

    assert_eq!(56, Move::square_notation_to_index("a1").unwrap());
    assert_eq!(57, Move::square_notation_to_index("b1").unwrap());
    assert_eq!(58, Move::square_notation_to_index("c1").unwrap());
    assert_eq!(59, Move::square_notation_to_index("d1").unwrap());
    assert_eq!(60, Move::square_notation_to_index("e1").unwrap());
    assert_eq!(61, Move::square_notation_to_index("f1").unwrap());
    assert_eq!(62, Move::square_notation_to_index("g1").unwrap());
    assert_eq!(63, Move::square_notation_to_index("h1").unwrap());
}

#[test]
fn index_to_notation() {
    for i in 0..63 {
        let notation = Move::index_to_square_notation(i).unwrap();
        let reversed = Move::square_notation_to_index(&notation[..]).unwrap();
        assert_eq!(i, reversed);
    }
}
