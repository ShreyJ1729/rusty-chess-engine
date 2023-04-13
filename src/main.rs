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

    // print all the diagonals and anti-diagonals

    for diagonal in DIAGONAL::iter() {
        // println!("{}:\n{}", diagonal, Bitboard::new(diagonal.bits()));
        // count number of 1s in diagonal and print it
        println!("{} {}", diagonal, diagonal.bits().count_ones());
    }

    for anti_diagonal in ANTIDIAGONAL::iter() {
        // println!(
        //     "{}:\n{}",
        //     anti_diagonal,
        //     Bitboard::new(anti_diagonal.bits())
        // );
        // count number of 1s in anti_diagonal and print it
        println!("{} {}", anti_diagonal, anti_diagonal.bits().count_ones());
    }

    lookup_table.build_moves();

    // for square in SQUARE::iter() {
    //     let true_moves = MoveGenerator::new().generate_bishop_moves(square, board.occupancy());
    //     let predicted_moves =
    //         Bitboard::new(lookup_table.bishop_moves(square, board.occupancy().bits()));
    //     println!(
    //         "{}\nmovegen:\n{}\npredicted_u64:{}\npredicted:\n{}\n--------------\n",
    //         square,
    //         true_moves,
    //         predicted_moves.bits(),
    //         predicted_moves
    //     );

    //     assert_eq!(true_moves, predicted_moves);
    // }

    for square in SQUARE::iter() {
        let true_moves = MoveGenerator::new().generate_rook_moves(square, board.occupancy());
        let predicted_moves =
            Bitboard::new(lookup_table.rook_moves(square, board.occupancy().bits()));
        println!(
            "{}\nmovegen:\n{}\npredicted_u64:{}\npredicted:\n{}\n--------------\n",
            square,
            true_moves,
            predicted_moves.bits(),
            predicted_moves
        );
        assert_eq!(true_moves, predicted_moves);
    }
}
