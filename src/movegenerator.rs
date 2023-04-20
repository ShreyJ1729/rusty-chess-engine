use crate::*;

use super::enums::COLOR;

#[derive(Debug, Clone, Copy)]
pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator {}
    }

    // --------------------------------------------------
    // ------------------- PAWN MOVES -------------------
    // --------------------------------------------------

    // todo add en passant and promotion
    pub fn generate_pawn_moves(&self, square: SQUARE, color: COLOR) -> Bitboard {
        let source = square.bits();

        let forward = match color {
            COLOR::WHITE => north(source),
            COLOR::BLACK => south(source),
        };

        let double_forward = match color {
            COLOR::WHITE => match square.rank() {
                RANK::Rank2 => north_north(source),
                _ => None,
            },
            COLOR::BLACK => match square.rank() {
                RANK::Rank7 => south_south(source),
                _ => None,
            },
            _ => None,
        };

        let left_capture = match color {
            COLOR::WHITE => north_west(source),
            COLOR::BLACK => south_west(source),
        };

        let right_capture = match color {
            COLOR::WHITE => north_east(source),
            COLOR::BLACK => south_east(source),
        };

        let targets = vec![forward, double_forward, left_capture, right_capture];

        let targets = targets.into_iter().fold(0, |acc, x| acc | x.unwrap_or(0));

        Bitboard::new(targets)
    }

    // ---------------------------------------------------
    // ------------------ LEAPING MOVES ------------------
    // ---------------------------------------------------

    // todo add castling
    pub fn generate_king_moves(&self, square: SQUARE) -> Bitboard {
        let source = square.bits();

        let targets = vec![
            north(source),
            north_east(source),
            east(source),
            south_east(source),
            south(source),
            south_west(source),
            west(source),
            north_west(source),
        ];

        let targets = targets.into_iter().fold(0, |acc, x| acc | x.unwrap_or(0));

        Bitboard::new(targets)
    }

    pub fn generate_knight_moves(&self, square: SQUARE) -> Bitboard {
        let source = square.bits();

        let targets = vec![
            north_north_east(source),
            east_east_north(source),
            east_east_south(source),
            south_south_east(source),
            south_south_west(source),
            west_west_south(source),
            west_west_north(source),
            north_north_west(source),
        ];

        let targets = targets.into_iter().fold(0, |acc, x| acc | x.unwrap_or(0));

        Bitboard::new(targets)
    }

    // ---------------------------------------------------
    // ------------------ SLIDING MOVES ------------------
    // ---------------------------------------------------

    pub fn generate_rook_moves(&self, square: SQUARE, occupancy: Bitboard) -> Bitboard {
        let source = square.bits();

        let mut targets: u64 = 0;

        let mut north_target = north(source);
        let mut south_target = south(source);
        let mut east_target = east(source);
        let mut west_target = west(source);

        while let Some(target) = north_target {
            targets |= target;
            north_target = north(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        // keep adding targets in direction until edge or piece encountered - for pieces, include that square
        while let Some(target) = south_target {
            targets |= target;
            south_target = south(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        while let Some(target) = east_target {
            targets |= target;
            east_target = east(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        while let Some(target) = west_target {
            targets |= target;
            west_target = west(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        Bitboard::new(targets)
    }

    pub fn generate_bishop_moves(&self, square: SQUARE, occupancy: Bitboard) -> Bitboard {
        let source = square.bits();

        let mut targets: u64 = 0;

        let mut north_east_target = north_east(source);
        let mut south_east_target = south_east(source);
        let mut north_west_target = north_west(source);
        let mut south_west_target = south_west(source);

        while let Some(target) = north_east_target {
            targets |= target;
            north_east_target = north_east(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        while let Some(target) = south_east_target {
            targets |= target;
            south_east_target = south_east(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        while let Some(target) = north_west_target {
            targets |= target;
            north_west_target = north_west(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        while let Some(target) = south_west_target {
            targets |= target;
            south_west_target = south_west(target);

            if occupancy.is_set_bits(target) {
                break;
            }
        }

        Bitboard::new(targets)
    }

    pub fn generate_queen_moves(&self, square: SQUARE, occupancy: Bitboard) -> Bitboard {
        self.generate_rook_moves(square, occupancy) | self.generate_bishop_moves(square, occupancy)
    }
}
