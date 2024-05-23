use crate::enums::SQUARE;
use strum_macros::Display;

pub const WKC_SQUARES: [SQUARE; 2] = [SQUARE::F1, SQUARE::G1];
pub const WQC_SQUARES: [SQUARE; 3] = [SQUARE::D1, SQUARE::C1, SQUARE::B1];
pub const BKC_SQUARES: [SQUARE; 2] = [SQUARE::F8, SQUARE::G8];
pub const BQC_SQUARES: [SQUARE; 3] = [SQUARE::D8, SQUARE::C8, SQUARE::B8];

pub const WKC_BITS: u64 = 0x0000000000000060;
pub const WQC_BITS: u64 = 0x000000000000000E;
pub const BKC_BITS: u64 = 0x6000000000000000;
pub const BQC_BITS: u64 = 0x0E00000000000000;

#[derive(Debug, Clone, Copy, Display)]
pub enum CASTLE {
    WhiteKingside = 0,
    WhiteQueenside = 1,
    BlackKingside = 2,
    BlackQueenside = 3,
}
