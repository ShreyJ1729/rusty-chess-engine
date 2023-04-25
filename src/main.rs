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
    std::env::set_var("RUST_BACKTRACE", "1");
    let lookup_table = LookupTable::new();

    let mut board = Board::starting_position(&lookup_table);
    println!("{}", board);

    let mut turn = COLOR::WHITE;
    let mut moves = board.generate_moves_for_color(turn);

    while moves.len() > 0 {
        let move_ = moves[rand::thread_rng().gen_range(0..moves.len())];
        println!("{} {}", turn, move_);
        board.make_move(move_);
        println!("{}", board);
        turn = turn.opposite();
        moves = board.generate_moves_for_color(turn);
    }

    println!("{}", board.to_fen());

    profile_moves_per_second();
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
