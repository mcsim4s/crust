#[cfg(test)]
use crate::engine::Engine;
use std::time::{Duration, SystemTime};
use crate::uci;

#[test]
fn searcher_test_mate_in_1_1() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("k7/6R1/8/8/8/8/8/6KR w - - 0 1")),
            moves: vec![],
        })
        .unwrap();

    let search = engine.search();
    assert_eq!("h1h8", search.to_notation())
}

#[test]
fn perf_test_1() {
    let mut engine = Engine::new();
    engine
        .execute_uci(uci::Command::SetPosition {
            position: uci::Position::Fen(String::from("r1bqkbnr/pppppppp/8/2P1P3/Pn1P4/3BBNN1/5PPP/RQ2R1K1 w k - 0 18")),
            moves: vec![],
        })
        .unwrap();

    let start = SystemTime::now();
    let _ = engine.search();
    assert!(start.elapsed().unwrap() < Duration::from_secs(10))
}