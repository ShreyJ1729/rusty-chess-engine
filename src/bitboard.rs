use crate::enums::*;
use std::fmt::{Display, Formatter, Result};
use std::ops::*;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard {
    bits: u64,
}

impl Bitboard {
    pub fn new(bits: u64) -> Bitboard {
        Bitboard { bits }
    }

    pub fn set(&mut self, index: usize) {
        self.bits |= 1 << index;
    }

    pub fn unset(&mut self, index: usize) {
        self.bits = self.bits & !(1 << index);
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn is_set(&self, index: usize) -> bool {
        self.bits & (1 << index) != 0
    }

    pub fn is_set_bits(&self, bits: u64) -> bool {
        self.bits & bits != 0
    }

    pub fn count(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn pop_lsb(&mut self) -> Option<usize> {
        if self.bits == 0 {
            return None;
        }

        let lsb = self.bits.trailing_zeros();
        self.unset(lsb as usize);
        Some(lsb as usize)
    }

    pub fn indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        let mut bits = self.bits;
        while bits != 0 {
            let lsb = bits.trailing_zeros();
            indices.push(lsb as usize);
            bits &= !(1 << lsb);
        }
        indices
    }

    pub fn get_squares(&self) -> Vec<SQUARE> {
        self.indices()
            .iter()
            .map(|&index| SQUARE::from(index))
            .collect()
    }

    pub fn any(&self) -> bool {
        self.bits != 0
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.bits |= rhs.bits;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.bits ^= rhs.bits;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.bits &= rhs.bits;
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard { bits: !self.bits }
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // since board is printed with rank 8 at the top, we need to iterate in reverse
        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;

            write!(f, "{} ", rank_index + 1)?;

            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;

                if self.is_set(index) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }

                write!(f, " ")?;
            }

            write!(f, "\n")?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}
