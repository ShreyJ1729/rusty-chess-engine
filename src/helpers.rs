use crate::*;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

pub fn build_move(source: SQUARE, target: SQUARE) -> Move {
    Move {
        source: source.bits(),
        target: target.bits(),
        // promotion,
    }
}

pub fn index_to_bits(index: usize) -> u64 {
    assert!(index < 64);
    1 << index
}

pub fn bits_to_index(bits: u64) -> usize {
    assert_eq!(bits.count_ones(), 1);
    bits.trailing_zeros() as usize
}

pub fn bits_to_square(bits: u64) -> SQUARE {
    assert_eq!(bits.count_ones(), 1);
    SQUARE::from(bits.trailing_zeros() as usize)
}

pub fn north(bits: u64) -> Option<u64> {
    ternary!(
        bits & RANK::Rank8.bits() == 0,
        Some(bits << DIRECTION::UpDown.bits()),
        None
    )
}

pub fn south(bits: u64) -> Option<u64> {
    ternary!(
        bits & RANK::Rank1.bits() == 0,
        Some(bits >> DIRECTION::UpDown.bits()),
        None
    )
}

pub fn east(bits: u64) -> Option<u64> {
    ternary!(
        bits & FILE::FileH.bits() == 0,
        Some(bits << DIRECTION::LeftRight.bits()),
        None
    )
}

pub fn west(bits: u64) -> Option<u64> {
    ternary!(
        bits & FILE::FileA.bits() == 0,
        Some(bits >> DIRECTION::LeftRight.bits()),
        None
    )
}

pub fn north_east(bits: u64) -> Option<u64> {
    north(bits).and_then(east)
}

pub fn north_west(bits: u64) -> Option<u64> {
    north(bits).and_then(west)
}

pub fn south_east(bits: u64) -> Option<u64> {
    south(bits).and_then(east)
}

pub fn south_west(bits: u64) -> Option<u64> {
    south(bits).and_then(west)
}

pub fn north_north(bits: u64) -> Option<u64> {
    north(bits).and_then(north)
}

pub fn south_south(bits: u64) -> Option<u64> {
    south(bits).and_then(south)
}

pub fn north_north_east(bits: u64) -> Option<u64> {
    north(bits).and_then(north_east)
}

pub fn north_north_west(bits: u64) -> Option<u64> {
    north(bits).and_then(north_west)
}

pub fn south_south_east(bits: u64) -> Option<u64> {
    south(bits).and_then(south_east)
}

pub fn south_south_west(bits: u64) -> Option<u64> {
    south(bits).and_then(south_west)
}

pub fn east_east_north(bits: u64) -> Option<u64> {
    north_east(bits).and_then(east)
}

pub fn west_west_north(bits: u64) -> Option<u64> {
    north_west(bits).and_then(west)
}

pub fn east_east_south(bits: u64) -> Option<u64> {
    south_east(bits).and_then(east)
}

pub fn west_west_south(bits: u64) -> Option<u64> {
    south_west(bits).and_then(west)
}
