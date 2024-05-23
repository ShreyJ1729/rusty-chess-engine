mod bits;
mod display;
mod endgame_detection;
mod fen;
mod move_generation;
mod move_validation;
mod perft;
mod piece_movement;

use crate::{bitboard::*, enums::*, lookup_table::*};

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub to_move: COLOR,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub en_passant_target: Option<SQUARE>,

    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

    pub lookup_table: &'a LookupTable,
}

impl<'a> Board<'a> {
    pub fn new(lookup_table: &'a LookupTable) -> Board<'a> {
        Self {
            to_move: COLOR::WHITE,
            castling_rights: CastlingRights::default(),
            halfmove_clock: 0,
            fullmove_number: 1,

            en_passant_target: None,

            white_pawns: Bitboard::default(),
            white_knights: Bitboard::default(),
            white_bishops: Bitboard::default(),
            white_rooks: Bitboard::default(),
            white_queens: Bitboard::default(),
            white_king: Bitboard::default(),

            black_pawns: Bitboard::default(),
            black_knights: Bitboard::default(),
            black_bishops: Bitboard::default(),
            black_rooks: Bitboard::default(),
            black_queens: Bitboard::default(),
            black_king: Bitboard::default(),

            lookup_table,
        }
    }
}
