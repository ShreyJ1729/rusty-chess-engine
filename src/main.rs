use crate::{
    bitboard::*, board::*, enums::*, helpers::*, lookuptable::*, movegenerator::*, r#move::*,
};
use rand::Rng;
use std::fmt::{Display, Formatter, Result};
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

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!("{}", board);

    let mut lookup_table = LookupTable::new();

    lookup_table.build_moves();
}
