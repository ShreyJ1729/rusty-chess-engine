use crate::{board::*, enums::*, helpers::*, movegenerator::*, r#move::*};
use std::fmt::{Display, Formatter, Result};
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

mod board;
mod enums;
mod helpers;
mod r#move;
mod movegenerator;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    println!("{}", board);

    let moves = board.generate_moves();

    // profile_num_moves_per_second();
    // profile_num_moves_generated();
}

fn profile_num_moves_per_second() {
    println!("profiling num_moves_per_second...");
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let num_moves = 100_000_000;
    let start = std::time::Instant::now();

    for _ in 0..(num_moves / 2) {
        board.make_move(build_move(SQUARE::A2, SQUARE::A4, None));
        board.make_move(build_move(SQUARE::A4, SQUARE::A2, None));
    }

    let end = std::time::Instant::now();
    let elapsed = end - start;
    let elapsed = elapsed.as_secs_f64();
    let num_moves_per_second = num_moves as f64 / elapsed;
    println!(
        "num_moves: {}, elapsed: {}, num_moves_per_second: {}",
        num_moves, elapsed, num_moves_per_second
    );
}

fn profile_num_moves_generated() {
    println!("profiling num_moves_generated...");
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let trials = 1_000;
    let start = std::time::Instant::now();
    let mut count = 0;

    for _ in 0..trials {
        count += board.generate_moves().len();
    }

    let end = std::time::Instant::now();
    let elapsed = end - start;
    let elapsed = elapsed.as_secs_f64();
    let num_moves_per_second = count as f64 / elapsed;
    println!(
        "trials: {}, elapsed: {}, count: {}, num_moves_per_second: {}",
        trials, elapsed, count, num_moves_per_second
    );
}
