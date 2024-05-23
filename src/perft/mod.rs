mod stockfish;

use crate::{
    board::*,
    lookup_table::{self, *},
};
use std::collections::HashMap;
use thousands::Separable;

const PASS: &str = "\x1b[32mPASS\x1b[0m";
const FAIL: &str = "\x1b[31mFAIL\x1b[0m";

pub fn load_perft_table(path: &str) -> HashMap<String, Vec<Option<u64>>> {
    let mut map = HashMap::new();
    let mut rdr = csv::Reader::from_path(path).unwrap();

    for result in rdr.records() {
        let record = result.expect("a CSV record");

        let fen = record.get(0).unwrap().to_string();
        let depth = record.get(1).unwrap().to_string();
        let key = format!("{}|{}", depth, fen);

        let data = record
            .iter()
            .skip(2)
            .map(|x| {
                if x.trim() == "None" {
                    None
                } else {
                    Some(
                        x.trim()
                            .parse::<u64>()
                            .expect(&format!("Failed to parse u64 from string '{}'", x)),
                    )
                }
            })
            .collect::<Vec<Option<u64>>>();

        map.insert(key, data);
    }

    map
}

pub fn run_perft_tests(max_depth: u8) {
    let lookup_table = LookupTable::new();
    let perft_table = load_perft_table("perft.csv");

    println!("len: {}", perft_table.len());

    for row in perft_table {
        // All rows have these fields
        let key = row.0;
        let depth = key.split('|').next().unwrap().parse::<u8>().unwrap();
        let fen = key.split('|').nth(1).unwrap().to_string();

        if depth > max_depth {
            continue;
        }

        // Not all rows have these fields, so we use Option
        let _nodes = row.1.get(0).copied().flatten();
        let _captures = row.1.get(1).copied().flatten();
        let _enp = row.1.get(2).copied().flatten();
        let _castles = row.1.get(3).copied().flatten();
        let _promo = row.1.get(4).copied().flatten();
        let _checks = row.1.get(5).copied().flatten();
        let _checkmates = row.1.get(6).copied().flatten();

        let (pass, nodes, captures, enp, castles, promo, checks) =
            run_fen(fen, depth, &lookup_table);

        print!("Nodes: {} | ", if pass { PASS } else { FAIL });

        if let Some(_captures) = _captures {
            print!(
                "Captures: {} | ",
                if captures == _captures { PASS } else { FAIL }
            );
        }
        if let Some(_enp) = _enp {
            print!("Enpassants: {} | ", if enp == _enp { PASS } else { FAIL });
        }
        if let Some(_castles) = _castles {
            print!(
                "Castles: {} | ",
                if castles == _castles { PASS } else { FAIL }
            );
        }
        if let Some(_promo) = _promo {
            print!(
                "Promotions: {} | ",
                if promo == _promo { PASS } else { FAIL }
            );
        }
        if let Some(_checks) = _checks {
            print!("Checks: {} | ", if checks == _checks { PASS } else { FAIL });
        }
        if let Some(_checkmates) = _checkmates {
            print!(
                "Checkmates: {}",
                if nodes == _checkmates { PASS } else { FAIL }
            );
        }
        println!();
    }
}

pub fn run_fen(
    fen: String,
    depth: u8,
    lookup_table: &LookupTable,
) -> (bool, u64, u64, u64, u64, u64, u64) {
    let mut board = Board::from_fen(&fen, &lookup_table);

    let mut rusty_perft = HashMap::new();
    let stockfish_perft = stockfish::get_perft_results(depth, board.to_fen());

    let start = std::time::Instant::now();
    let (nodes, captures, enpassants, castles, promotions, checks) =
        board.perft(depth, depth, None, &mut rusty_perft);
    let elapsed = start.elapsed().as_secs_f64();

    println!("--------------------------------------------------------------------");
    println!(
        "depth={} for {}:\nNodes: {}, Captures: {}, Enpassants: {}, Castles: {}, Promotions: {}, Checks: {}",
        depth, fen, nodes.separate_with_commas(), captures.separate_with_commas(), enpassants.separate_with_commas(), castles.separate_with_commas(), promotions.separate_with_commas(), checks.separate_with_commas(),
    );
    println!(
        "Finished in {:.2} seconds at {} nodes/second",
        elapsed,
        ((nodes as f64 / elapsed) as i32).separate_with_commas()
    );
    println!("--------------------------------------------------------------------");

    // now we compare the two maps
    let mut total = 0;
    let mut pass = true;

    // ensure stockfish moves are in computed moves
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
    // ensure no moves are in computed moves that aren't in stockfish moves
    for (key, value) in &rusty_perft {
        if !stockfish_perft.contains_key(key) {
            println!("{}: {} not found in stockfish_perft", key, value);
            pass = false;
        }
    }

    (
        pass, nodes, captures, enpassants, castles, promotions, checks,
    )
}
