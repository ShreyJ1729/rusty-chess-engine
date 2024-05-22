use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, Display, Clone, Copy, PartialEq, EnumIter)]
pub enum COLOR {
    WHITE = 0,
    BLACK = 1,
}

impl COLOR {
    pub fn index(&self) -> usize {
        match self {
            COLOR::WHITE => 0,
            COLOR::BLACK => 1,
        }
    }

    pub fn opposite(&self) -> COLOR {
        match self {
            COLOR::WHITE => COLOR::BLACK,
            COLOR::BLACK => COLOR::WHITE,
        }
    }

    pub fn to_fen(&self) -> char {
        match self {
            COLOR::WHITE => 'w',
            COLOR::BLACK => 'b',
        }
    }
}
