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

    // let mut turn = COLOR::WHITE;

    // while board.occupancy().count() > 2 {
    //     let moves = board.generate_moves_for_color(turn);
    //     let move_ = moves[rand::thread_rng().gen_range(0..moves.len())];
    //     println!("{} {}", turn, move_);
    //     board.make_move(move_);
    //     println!("{}", board);
    //     turn = turn.opposite();
    // }

    // println!(
    //     "total moves: {}",
    //     board.move_history.unwrap_or(vec![]).len()
    // );

    profile_moves_per_second(10);
}

fn profile_moves_per_second(num_seconds: u64) {
    let lookup_table = LookupTable::new();
    let board = Board::starting_position(&lookup_table);
    let mut moves = board.generate_moves_for_color(COLOR::WHITE);

    let start = std::time::Instant::now();
    let mut count = 0;

    while start.elapsed().as_secs() < 10 {
        moves = board.generate_moves_for_color(COLOR::WHITE);
        count += 1;
    }

    println!(
        "{} moves per second",
        moves.len() * count / num_seconds as usize
    );
}
