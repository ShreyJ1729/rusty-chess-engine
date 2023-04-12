use crate::{board::*, enums::*, helpers::*, movegenerator::*, movelookup::*, r#move::*};
use std::fmt::{Display, Formatter, Result};
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

mod board;
mod enums;
mod helpers;
mod r#move;
mod movegenerator;
mod movelookup;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    println!("{}", board);
}
