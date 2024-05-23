mod args;
mod bitboard;
mod board;
mod enums;
mod helpers;
mod lookup_table;
mod r#move;
mod move_validator;
mod stockfish;

use args::*;
use board::*;
use lookup_table::*;
use std::collections::HashMap;

fn main() {
    let args = Args::parse_args();

    match args.debug {
        true => std::env::set_var("RUST_BACKTRACE", "1"),
        false => {}
    }

    match args.mode {
        EngineMode::PERFT => run_perft(args.depth, args.fen),
    }
}

fn run_perft(depth: u8, fen: String) {
    let lookup_table = LookupTable::new();
    let mut board = Board::from_fen(&fen, &lookup_table);

    println!("{}", board);

    let mut rusty_perft = HashMap::new();
    let stockfish_perft = stockfish::get_perft_results(depth, board.to_fen());

    let start = std::time::Instant::now();

    let (nodes, captures, castles, enpassants, promotions, checks) =
        board.perft(depth, depth, &mut rusty_perft);

    let elapsed = start.elapsed().as_secs_f64();

    println!("--------------------------------------------------------------------");
    println!(
        "Computed for depth={}:\nNodes: {}, Captures: {}, Castles: {}, Enpassants: {}, Promotions: {}, Checks: {}",
        depth, nodes, captures, castles, enpassants, promotions, checks
    );
    println!(
        "Finished in {:.1} seconds at {:.0} nodes/second",
        elapsed,
        nodes as f64 / elapsed
    );
    println!("--------------------------------------------------------------------");

    // now we compare the two maps
    let mut total = 0;
    let mut pass = true;

    // ensure stockfish moves in computed moves
    for (key, value) in &stockfish_perft {
        total += value;
        if let Some(rusty_value) = rusty_perft.get(key) {
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
            println!("{} not found in rusty_perft", key);
            pass = false;
        }
    }
    // ensure no moves in computed moves that aren't in stockfish moves
    for (key, value) in &rusty_perft {
        if !stockfish_perft.contains_key(key) {
            println!("{}: {} not found in stockfish_perft", key, value);
            pass = false;
        }
    }

    println!("stockfish total nodes:\t{}", total);
    println!("rusty total nodes:\t{}", nodes);

    println!("{}", if pass { "PASS" } else { "FAIL" });
}
