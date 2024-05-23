mod args;
mod bitboard;
mod board;
mod enums;
mod helpers;
mod lookup_table;
mod r#move;
mod move_validator;
mod perft;

use args::*;

fn main() {
    let args = Args::parse_args();

    match args.debug {
        true => std::env::set_var("RUST_BACKTRACE", "1"),
        false => {}
    }

    match args.mode {
        EngineMode::PERFT => perft::run_perft_tests(args.max_depth),
    }
}
