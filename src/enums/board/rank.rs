use std::ops::{Add, Sub};
use strum_macros::Display;
use strum_macros::EnumIter;

// ranks and files are defined in the following way so that rank * 8 + file = square
#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq, PartialOrd, Eq, Ord)]
pub enum RANK {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

impl RANK {
    pub fn bits(&self) -> u64 {
        match self {
            RANK::Rank1 => 0xFF,     // 0b11111111
            RANK::Rank2 => 0xFF00,   // 0b1111111100000000
            RANK::Rank3 => 0xFF0000, // ...
            RANK::Rank4 => 0xFF000000,
            RANK::Rank5 => 0xFF00000000,
            RANK::Rank6 => 0xFF0000000000,
            RANK::Rank7 => 0xFF000000000000,
            RANK::Rank8 => 0xFF00000000000000,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn to_fen(&self) -> char {
        match self {
            RANK::Rank1 => '1',
            RANK::Rank2 => '2',
            RANK::Rank3 => '3',
            RANK::Rank4 => '4',
            RANK::Rank5 => '5',
            RANK::Rank6 => '6',
            RANK::Rank7 => '7',
            RANK::Rank8 => '8',
        }
    }
}

impl Sub for RANK {
    type Output = i8;

    fn sub(self, other: RANK) -> i8 {
        self.index() as i8 - other.index() as i8
    }
}

impl Add for RANK {
    type Output = i8;

    fn add(self, other: RANK) -> i8 {
        self.index() as i8 + other.index() as i8
    }
}
