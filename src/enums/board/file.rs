use std::ops::{Add, Sub};
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq, PartialOrd, Eq)]
pub enum FILE {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}

impl FILE {
    pub fn bits(&self) -> u64 {
        match self {
            FILE::FileA => 0x0101010101010101, // 0b0000000100000001000000010000000100000001000000010000000100000001
            FILE::FileB => 0x0202020202020202, // 0b0000001000000010000000100000001000000010000000100000001000000010
            FILE::FileC => 0x0404040404040404, // ...
            FILE::FileD => 0x0808080808080808,
            FILE::FileE => 0x1010101010101010,
            FILE::FileF => 0x2020202020202020,
            FILE::FileG => 0x4040404040404040,
            FILE::FileH => 0x8080808080808080,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn to_fen(&self) -> char {
        match self {
            FILE::FileA => 'a',
            FILE::FileB => 'b',
            FILE::FileC => 'c',
            FILE::FileD => 'd',
            FILE::FileE => 'e',
            FILE::FileF => 'f',
            FILE::FileG => 'g',
            FILE::FileH => 'h',
        }
    }
}

impl Add for FILE {
    type Output = i8;

    fn add(self, other: FILE) -> i8 {
        self.index() as i8 + other.index() as i8
    }
}

impl Sub for FILE {
    type Output = i8;

    fn sub(self, other: FILE) -> i8 {
        self.index() as i8 - other.index() as i8
    }
}
