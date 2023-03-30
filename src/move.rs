use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub source: u64,
    pub target: u64,
    pub promotion: Option<PIECE>,
    // en_passant: bool,
    // double_push: bool,
    // castle: Option<CASTLE>,
    // piece: PIECE,
    // capture: Option<PIECE>,
}

impl Move {
    pub fn new(source: u64, target: u64, promotion: Option<PIECE>) -> Move {
        Move {
            source,
            target,
            promotion: None,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}->{}",
            u64_to_square(self.source),
            u64_to_square(self.target)
        )
    }
}
