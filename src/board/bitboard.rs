use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Bitboard {
    bits: u64,
}

impl Bitboard {
    pub fn new(bits: Option<u64>) -> Bitboard {
        match bits {
            Some(bits) => Bitboard { bits },
            None => Bitboard { bits: 0 },
        }
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
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard {
            bits: self.bits | rhs.bits,
        }
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

impl Default for Bitboard {
    fn default() -> Self {
        Self::new(None)
    }
}
