use crate::board::*;
use crate::cell::*;
use crate::enums::*;
use crate::r#move::*;

mod board;
mod cell;
mod enums;
mod r#move;

fn main() {
    println!("Rusty Chess Engine");
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    board.print();

    let moves = board.generate_moves(COLOR::WHITE);

    for m in moves {
        println!("{:?}", m);
    }
}
