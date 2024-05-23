use clap::{Parser, ValueEnum};
use strum_macros::Display;

#[derive(Parser, Debug, Clone, Display, ValueEnum, PartialEq, Eq)]
pub enum EngineMode {
    PERFT,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "perft")]
    pub mode: EngineMode,

    #[arg(
        short,
        long,
        default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    )]
    pub fen: String,

    #[arg(short, long, default_value_t = 6)]
    pub max_depth: u8,

    #[arg(long, default_value_t = true)]
    pub debug: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
