#[cfg(test)]
use crate::{engine::Engine, uci};

#[test]
fn perf_test_1() {
    let engine = Engine::new();
    assert_eq!(20, engine.performance_test(1));
    assert_eq!(400, engine.performance_test(2));
    assert_eq!(8_902, engine.performance_test(3));
    assert_eq!(197_281, engine.performance_test(4));
    //    assert_eq!(4_865_609, engine.performance_test(5));
}

#[test]
fn perf_test_2() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0")),
            moves: vec![],
        })
        .unwrap();

    assert_eq!(48, engine.performance_test(1));
    assert_eq!(2_039, engine.performance_test(2));
    assert_eq!(97_862, engine.performance_test(3));
    //    assert_eq!(4_085_603, engine.performance_test(4));
    //    assert_eq!(193_690_690, engine.performance_test(5));
}

#[test]
fn perf_test_3() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0")),
            moves: vec![],
        })
        .unwrap();

    assert_eq!(14, engine.performance_test(1));
    assert_eq!(191, engine.performance_test(2));
    assert_eq!(2_812, engine.performance_test(3));
    assert_eq!(43_238, engine.performance_test(4));
    assert_eq!(674_624, engine.performance_test(5));
    //    assert_eq!(11_030_083, engine.performance_test(6));
}

#[test]
fn perf_test_4() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")),
            moves: vec![],
        })
        .unwrap();

    assert_eq!(6, engine.performance_test(1));
    assert_eq!(264, engine.performance_test(2));
    assert_eq!(9_467, engine.performance_test(3));
    assert_eq!(422_333, engine.performance_test(4));
    //    assert_eq!(15_833_292, engine.performance_test(5));
    //    assert_eq!(706_045_033, engine.performance_test(6));
}

#[test]
fn perf_test_5() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")),
            moves: vec![],
        })
        .unwrap();

    assert_eq!(44, engine.performance_test(1));
    assert_eq!(1_486, engine.performance_test(2));
    assert_eq!(62_379, engine.performance_test(3));
    //    assert_eq!(2_103_487, engine.performance_test(4));
    //    assert_eq!(89_941_194, engine.performance_test(5));
}
