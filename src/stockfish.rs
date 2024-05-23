use std::collections::HashMap;
use std::io::{Read, Write};

pub fn get_perft_results(depth: u8, fen: String) -> HashMap<String, u64> {
    let stockfish = std::process::Command::new("stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start Stockfish");

    let mut stdin = stockfish.stdin.expect("Failed to open stdin");
    let mut stdout = stockfish.stdout.expect("Failed to open stdout");

    let stockfish_command = format!("position fen {}\ngo perft {}\nquit\n", fen, depth);
    stdin
        .write_all(stockfish_command.as_bytes())
        .expect("Failed to write to stdin");

    let mut buffer = String::new();

    stdout
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdout");

    let mut lines = buffer.lines();
    lines.next();

    let mut stockfish_perft = HashMap::new();
    for line in lines {
        if line == "" {
            break;
        }
        let mut split = line.split(":");
        let move_str = split.next().unwrap();
        let count = split.next().unwrap().trim().parse::<u64>().unwrap();
        stockfish_perft.insert(move_str.to_string(), count);
    }

    stockfish_perft
}
