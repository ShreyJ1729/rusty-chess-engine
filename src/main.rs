use crate::{
    args::*, bitboard::*, board::*, enums::*, helpers::*, lookuptable::*, movegenerator::*,
    movevalidator::*, r#move::*,
};
use rand::Rng;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::io::prelude::*;
use std::ops::*;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

mod args;
mod bitboard;
mod board;
mod enums;
mod helpers;
mod lookuptable;
mod r#move;
mod movegenerator;
mod movevalidator;

fn main() {
    let args = Args::parse_args();

    match args.debug {
        true => std::env::set_var("RUST_BACKTRACE", "1"),
        false => {}
    }

    match args.mode {
        EngineMode::PERFT => run_perft(args.depth, args.fen),
        EngineMode::UCI => run_uci(args.fen),
        EngineMode::BESTMOVE => get_bestmove(args.fen),
    }
}

fn run_uci(fen: String) {
    let lookuptable = LookupTable::new();
    let mut board = Board::from_fen(&fen, &lookuptable);

    todo!("UCI not implemented yet");
}

fn get_bestmove(fen: String) {
    let lookuptable = LookupTable::new();
    let mut board = Board::from_fen(&fen, &lookuptable);

    todo!("BESTMOVE not implemented yet");
}

fn run_perft(depth: u8, fen: String) {
    let lookup_table = LookupTable::new();
    let mut board = Board::starting_position(&lookup_table);

    println!("{}", board);

    // hashmap of move --> # nodes for stockfish and rusty
    let mut rusty_moves = HashMap::new();
    let stockfish_moves = get_stockfish_perft(depth, board.to_fen());

    let start = std::time::Instant::now();

    // run perft
    let (nodes, captures, castles, enpassants, promotions) =
        board.perft(depth, depth, &mut rusty_moves);

    let elapsed = start.elapsed().as_secs_f64();

    println!("--------------------------------------------------------------------");
    println!(
        "Computed for depth={}:\nNodes: {}, Captures: {}, Castles: {}, Enpassants: {}, Promotions: {}",
        depth, nodes, captures, castles, enpassants, promotions
    );
    println!(
        "Finished in {} seconds at {} nodes/second",
        elapsed,
        nodes as f64 / elapsed
    );
    println!("--------------------------------------------------------------------");

    // now we compare the two maps
    let mut total = 0;
    let mut pass = true;

    // ensure stockfish moves in computed moves
    for (key, value) in &stockfish_moves {
        total += value;
        if let Some(rusty_value) = rusty_moves.get(key) {
            if *value != *rusty_value {
                println!(
                    "Mismatch for {}, stockfish: {}, rusty: {}, diff: {}",
                    key,
                    value,
                    rusty_value,
                    *rusty_value as i32 - *value as i32
                );
                pass = false;
            }
        } else {
            println!("{} not found in rusty_moves", key);
            pass = false;
        }
    }
    // ensure no moves in computed moves that aren't in stockfish moves
    for (key, value) in &rusty_moves {
        if !stockfish_moves.contains_key(key) {
            println!("{}: {} not found in stockfish_moves", key, value);
            pass = false;
        }
    }

    println!("stockfish total nodes:\t{}", total);
    println!("rusty total nodes:\t{}", nodes);

    println!("{}", if pass { "PASS" } else { "FAIL" });
}

fn get_stockfish_perft(depth: u8, fen: String) -> HashMap<String, u64> {
    let mut stockfish = std::process::Command::new("stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start Stockfish");

    let mut stdin = stockfish.stdin.take().unwrap();

    let stockfish_command = format!("position fen {}\ngo perft {}\nquit\n", fen, depth);
    println!("{}", stockfish_command);
    stdin
        .write_all(stockfish_command.as_bytes())
        .expect("Failed to write to stdin");

    let mut stdout = stockfish.stdout.take().unwrap();

    let mut buffer = String::new();

    stdout
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdout");

    println!("{}", buffer);

    let mut lines = buffer.lines();
    // ignore first line
    lines.next();
    // take each of the next line until 2 newlines, and parse by splitting on :
    let mut stockfish_moves: HashMap<String, u64> = HashMap::new();
    for line in lines {
        if line == "" {
            break;
        }
        let mut split = line.split(":");
        let move_str = split.next().unwrap();
        let count = split.next().unwrap().trim().parse::<u64>().unwrap();
        stockfish_moves.insert(move_str.to_string(), count);
    }

    stockfish_moves
}
