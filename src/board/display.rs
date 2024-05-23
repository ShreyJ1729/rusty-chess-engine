use crate::board::Board;
use crate::enums::{FILE, RANK};
use std::fmt::{Display, Formatter, Result};
use strum::IntoEnumIterator;

impl Display for Board<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;

            write!(f, "{} ", rank_index + 1)?;

            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;
                let piece = self.piece_at_index(index);
                let c = match piece.not_empty() {
                    true => piece as u8 as char,
                    false => ' ',
                };

                write!(f, "{} ", c)?;
            }
            write!(f, "\n")?;
        }
        writeln!(f, "  a b c d e f g h")
    }
}
