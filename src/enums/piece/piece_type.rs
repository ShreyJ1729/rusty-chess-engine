use crate::enums::*;
use strum_macros::Display;

pub const PROMOTION_OPTIONS: [PieceType; 4] = [
    PieceType::KNIGHT,
    PieceType::BISHOP,
    PieceType::ROOK,
    PieceType::QUEEN,
];

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
    EMPTY,
}

impl PieceType {
    pub fn for_color(&self, color: COLOR) -> PIECE {
        match color {
            COLOR::WHITE => match self {
                PieceType::PAWN => PIECE::WhitePawn,
                PieceType::KNIGHT => PIECE::WhiteKnight,
                PieceType::BISHOP => PIECE::WhiteBishop,
                PieceType::ROOK => PIECE::WhiteRook,
                PieceType::QUEEN => PIECE::WhiteQueen,
                PieceType::KING => PIECE::WhiteKing,
                PieceType::EMPTY => panic!("Empty piece has no color"),
            },
            COLOR::BLACK => match self {
                PieceType::PAWN => PIECE::BlackPawn,
                PieceType::KNIGHT => PIECE::BlackKnight,
                PieceType::BISHOP => PIECE::BlackBishop,
                PieceType::ROOK => PIECE::BlackRook,
                PieceType::QUEEN => PIECE::BlackQueen,
                PieceType::KING => PIECE::BlackKing,
                PieceType::EMPTY => panic!("Empty piece has no color"),
            },
        }
    }
}
