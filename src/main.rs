use crate::{
    bitboard::*, board::*, enums::*, helpers::*, lookuptable::*, movegenerator::*,
    movevalidator::*, r#move::*,
};
use rand::Rng;
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::io::prelude::*;
use std::ops::*;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

mod bitboard;
mod board;
mod enums;
mod helpers;
mod lookuptable;
mod r#move;
mod movegenerator;
mod movevalidator;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let lookup_table = LookupTable::new();

    let mut board = Board::starting_position(&lookup_table);

    // e2e3
    board.make_move(Move::new(SQUARE::E2, SQUARE::E3, None, None, None, false));
    let depth = 3;

    let (nodes, captures, castles, enpassants, promotions) = board.perft(depth, depth);
    println!("--------------------------------------------------------------------");
    println!(
        "Total for depth={}:\nNodes: {}, Captures: {}, Castles: {}, Enpassants: {}, Promotions: {}",
        depth, nodes, captures, castles, enpassants, promotions
    );
    println!("--------------------------------------------------------------------");

    // profile_moves_per_second();
}

fn profile_moves_per_second() {
    println!("Profiling moves per second...");
    let lookup_table = LookupTable::new();
    let board = Board::starting_position(&lookup_table);
    let mut moves = board.generate_moves_for_color(COLOR::WHITE);

    let start = std::time::Instant::now();
    let mut count = 0;

    while start.elapsed().as_secs() < 1 {
        moves = board.generate_moves_for_color(COLOR::WHITE);
        count += 1;
    }

    println!("{} moves/second generated", moves.len() * count);
}
