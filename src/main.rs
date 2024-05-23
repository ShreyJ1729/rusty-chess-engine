mod args;
mod bitboard;
mod board;
mod enums;
mod helpers;
mod lookup_table;
mod r#move;
mod perft;

use args::*;

pub fn time_fn<T>(body: impl FnOnce() -> T) -> (T, f64) {
    let start_time = std::time::Instant::now();
    let result = body();
    let total_time =
        start_time.elapsed().as_secs() as f64 + start_time.elapsed().subsec_nanos() as f64 / 1e9;
    (result, total_time)
}

fn main() {
    let args = Args::parse_args();

    match args.debug {
        true => std::env::set_var("RUST_BACKTRACE", "1"),
        false => {}
    }

    match args.mode {
        EngineMode::PERFT => perft::run_perft_tests(args.depth),
    }
}
