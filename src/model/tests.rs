#[cfg(test)]
use crate::{engine::Engine, model::*, uci};

#[test]
fn construct_start_board() {
    let start_board = Board::new();
    assert_eq!(pieces::new(ROOK, BLACK), start_board.squares[0]);
    assert_eq!(pieces::new(KNIGHT, BLACK), start_board.squares[1]);
    assert_eq!(pieces::new(BISHOP, BLACK), start_board.squares[2]);
    assert_eq!(pieces::new(QUEEN, BLACK), start_board.squares[3]);
    assert_eq!(pieces::new(KING, BLACK), start_board.squares[4]);
    assert_eq!(pieces::new(BISHOP, BLACK), start_board.squares[5]);
    assert_eq!(pieces::new(KNIGHT, BLACK), start_board.squares[6]);
    assert_eq!(pieces::new(ROOK, BLACK), start_board.squares[7]);
    for i in 8..15 {
        assert_eq!(pieces::new(PAWN, BLACK), start_board.squares[i]);
    }
    for i in 16..47 {
        assert_eq!(pieces::NONE, start_board.squares[i]);
    }
    for i in 48..55 {
        assert_eq!(pieces::new(PAWN, WHITE), start_board.squares[i]);
    }
    assert_eq!(pieces::new(ROOK, WHITE), start_board.squares[56]);
    assert_eq!(pieces::new(KNIGHT, WHITE), start_board.squares[57]);
    assert_eq!(pieces::new(BISHOP, WHITE), start_board.squares[58]);
    assert_eq!(pieces::new(QUEEN, WHITE), start_board.squares[59]);
    assert_eq!(pieces::new(KING, WHITE), start_board.squares[60]);
    assert_eq!(pieces::new(BISHOP, WHITE), start_board.squares[61]);
    assert_eq!(pieces::new(KNIGHT, WHITE), start_board.squares[62]);
    assert_eq!(pieces::new(ROOK, WHITE), start_board.squares[63]);

    assert!(start_board.white_is_active);
    assert!(start_board.castle_white_king);
    assert!(start_board.castle_white_queen);
    assert!(start_board.castle_black_king);
    assert!(start_board.castle_black_queen);
}

#[test]
fn notation_to_index() {
    assert_eq!(0, util::square_notation_to_index("a8").unwrap());
    assert_eq!(1, util::square_notation_to_index("b8").unwrap());
    assert_eq!(2, util::square_notation_to_index("c8").unwrap());
    assert_eq!(3, util::square_notation_to_index("d8").unwrap());
    assert_eq!(4, util::square_notation_to_index("e8").unwrap());
    assert_eq!(5, util::square_notation_to_index("f8").unwrap());
    assert_eq!(6, util::square_notation_to_index("g8").unwrap());
    assert_eq!(7, util::square_notation_to_index("h8").unwrap());

    assert_eq!(8, util::square_notation_to_index("a7").unwrap());
    assert_eq!(9, util::square_notation_to_index("b7").unwrap());
    assert_eq!(10, util::square_notation_to_index("c7").unwrap());
    assert_eq!(11, util::square_notation_to_index("d7").unwrap());
    assert_eq!(12, util::square_notation_to_index("e7").unwrap());
    assert_eq!(13, util::square_notation_to_index("f7").unwrap());
    assert_eq!(14, util::square_notation_to_index("g7").unwrap());
    assert_eq!(15, util::square_notation_to_index("h7").unwrap());

    assert_eq!(16, util::square_notation_to_index("a6").unwrap());
    assert_eq!(17, util::square_notation_to_index("b6").unwrap());
    assert_eq!(18, util::square_notation_to_index("c6").unwrap());
    assert_eq!(19, util::square_notation_to_index("d6").unwrap());
    assert_eq!(20, util::square_notation_to_index("e6").unwrap());
    assert_eq!(21, util::square_notation_to_index("f6").unwrap());
    assert_eq!(22, util::square_notation_to_index("g6").unwrap());
    assert_eq!(23, util::square_notation_to_index("h6").unwrap());

    assert_eq!(24, util::square_notation_to_index("a5").unwrap());
    assert_eq!(25, util::square_notation_to_index("b5").unwrap());
    assert_eq!(26, util::square_notation_to_index("c5").unwrap());
    assert_eq!(27, util::square_notation_to_index("d5").unwrap());
    assert_eq!(28, util::square_notation_to_index("e5").unwrap());
    assert_eq!(29, util::square_notation_to_index("f5").unwrap());
    assert_eq!(30, util::square_notation_to_index("g5").unwrap());
    assert_eq!(31, util::square_notation_to_index("h5").unwrap());

    assert_eq!(32, util::square_notation_to_index("a4").unwrap());
    assert_eq!(33, util::square_notation_to_index("b4").unwrap());
    assert_eq!(34, util::square_notation_to_index("c4").unwrap());
    assert_eq!(35, util::square_notation_to_index("d4").unwrap());
    assert_eq!(36, util::square_notation_to_index("e4").unwrap());
    assert_eq!(37, util::square_notation_to_index("f4").unwrap());
    assert_eq!(38, util::square_notation_to_index("g4").unwrap());
    assert_eq!(39, util::square_notation_to_index("h4").unwrap());

    assert_eq!(40, util::square_notation_to_index("a3").unwrap());
    assert_eq!(41, util::square_notation_to_index("b3").unwrap());
    assert_eq!(42, util::square_notation_to_index("c3").unwrap());
    assert_eq!(43, util::square_notation_to_index("d3").unwrap());
    assert_eq!(44, util::square_notation_to_index("e3").unwrap());
    assert_eq!(45, util::square_notation_to_index("f3").unwrap());
    assert_eq!(46, util::square_notation_to_index("g3").unwrap());
    assert_eq!(47, util::square_notation_to_index("h3").unwrap());

    assert_eq!(48, util::square_notation_to_index("a2").unwrap());
    assert_eq!(49, util::square_notation_to_index("b2").unwrap());
    assert_eq!(50, util::square_notation_to_index("c2").unwrap());
    assert_eq!(51, util::square_notation_to_index("d2").unwrap());
    assert_eq!(52, util::square_notation_to_index("e2").unwrap());
    assert_eq!(53, util::square_notation_to_index("f2").unwrap());
    assert_eq!(54, util::square_notation_to_index("g2").unwrap());
    assert_eq!(55, util::square_notation_to_index("h2").unwrap());

    assert_eq!(56, util::square_notation_to_index("a1").unwrap());
    assert_eq!(57, util::square_notation_to_index("b1").unwrap());
    assert_eq!(58, util::square_notation_to_index("c1").unwrap());
    assert_eq!(59, util::square_notation_to_index("d1").unwrap());
    assert_eq!(60, util::square_notation_to_index("e1").unwrap());
    assert_eq!(61, util::square_notation_to_index("f1").unwrap());
    assert_eq!(62, util::square_notation_to_index("g1").unwrap());
    assert_eq!(63, util::square_notation_to_index("h1").unwrap());
}

#[test]
fn index_to_notation() {
    for i in 0..63 {
        let notation = util::index_to_square_notation(i).unwrap();
        let reversed = util::square_notation_to_index(&notation[..]).unwrap();
        assert_eq!(i, reversed);
    }
}

#[test]
fn disable_castle() {
    let mut engine = Engine::new();
    let command = uci::Command::parse("position startpos moves g1h3 g7g5 e2e3 f7f5 f1d3 g5g4 e1g1 d7d6 d3a6 c7c6 g2g3 e8f7 b1c3 b8a6 g1g2 g8f6 d1g4 c8e6 g4e2 e6b3 a2a3 f6e8 h3g5 f7f6 g5f7 d8b6 f7g5").unwrap();
    engine.execute_uci(command).unwrap();
    assert!(!engine.board.castle_white_king);
    assert!(!engine.board.castle_white_queen);
    assert!(!engine.board.castle_black_king);
    assert!(!engine.board.castle_black_queen);
}
