use crate::{board::*, lookup_table::*};
use ascii_table::AsciiTable;
use core::ascii;
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

pub fn run_perft_tests(depth: u8) {
    let lookup_table = LookupTable::new();
    let perft_table = load_perft_table("perft.csv");

    let mut all_pass = true;

    for row in perft_table {
        // All rows have these fields
        let key = row.0;
        let _depth = key.split('|').next().unwrap().parse::<u8>().unwrap();
        let fen = key.split('|').nth(1).unwrap().to_string();

        // Skip rows that don't match the requested depth
        if depth != _depth {
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

        let (elapsed, nodes, captures, enp, castles, promo, checks, checkmates) =
            get_perft_result_for_fen(fen.clone(), depth, &lookup_table);

        println!("{}", "-".repeat(80));
        println!("{}\nDepth = {}", fen, depth);
        println!("Metric \t\t Calculated \t Expected");
        println!("------ \t\t ---------- \t --------");

        let (mut c_pass, mut enp_pass, mut ca_pass, mut p_pass, mut ch_pass, mut cm_pass) =
            (true, true, true, true, true, true);

        let n_pass = nodes == _nodes.unwrap();
        println!(
            "Nodes:\t\t {} \t\t {} \t\t {}",
            nodes,
            _nodes.unwrap(),
            if n_pass { PASS } else { FAIL },
        );

        if let Some(_captures) = _captures {
            c_pass = captures == _captures;
            println!(
                "Captures:\t {} \t\t {} \t\t {}",
                captures,
                _captures,
                if c_pass { PASS } else { FAIL },
            );
        }
        if let Some(_enp) = _enp {
            enp_pass = enp == _enp;
            println!(
                "Enpassants:\t {} \t\t {} \t\t {}",
                enp,
                _enp,
                if enp_pass { PASS } else { FAIL },
            );
        }
        if let Some(_castles) = _castles {
            ca_pass = castles == _castles;
            println!(
                "Castles:\t {} \t\t {} \t\t {}",
                castles,
                _castles,
                if ca_pass { PASS } else { FAIL },
            );
        }
        if let Some(_promo) = _promo {
            p_pass = promo == _promo;
            println!(
                "Promotions:\t {} \t\t {} \t\t {}",
                promo,
                _promo,
                if p_pass { PASS } else { FAIL },
            );
        }
        if let Some(_checks) = _checks {
            ch_pass = checks == _checks;
            println!(
                "Checks:\t\t {} \t\t {} \t\t {}",
                checks,
                _checks,
                if ch_pass { PASS } else { FAIL },
            );
        }
        if let Some(_checkmates) = _checkmates {
            cm_pass = checkmates == _checkmates;
            println!(
                "Checkmates:\t {} \t\t {} \t\t {}",
                checkmates,
                _checkmates,
                if cm_pass { PASS } else { FAIL },
            );
        }

        if !n_pass || !c_pass || !enp_pass || !ca_pass || !p_pass || !ch_pass || !cm_pass {
            all_pass = false;
        }

        println!(
            "\nFinished in {:.2} seconds at {} nodes/second",
            elapsed,
            ((nodes as f64 / elapsed) as i32).separate_with_commas()
        );
    }

    println!("{}", "-".repeat(80));
    println!("All tests passed: {}", if all_pass { PASS } else { FAIL });
}

pub fn get_perft_result_for_fen(
    fen: String,
    depth: u8,
    lookup_table: &LookupTable,
) -> (f64, u64, u64, u64, u64, u64, u64, u64) {
    let mut board = Board::from_fen(&fen, &lookup_table);
    let start = std::time::Instant::now();

    let (nodes, captures, enpassants, castles, promotions, checks, checkmates) =
        board.perft(depth, depth, false);

    let elapsed = start.elapsed().as_secs_f64();

    (
        elapsed, nodes, captures, enpassants, castles, promotions, checks, checkmates,
    )
}
