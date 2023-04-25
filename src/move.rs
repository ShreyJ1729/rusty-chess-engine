use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub source: SQUARE,
    pub target: SQUARE,
    pub promotion: Option<PieceType>,
    pub castling: Option<CASTLE>,
    // en_passant: bool,
    // double_push: bool,
    // castle: Option<CASTLE>,
    // piece: PIECE,
    // capture: Option<PIECE>,
}

impl Move {
    pub fn new(
        source: SQUARE,
        target: SQUARE,
        promotion: Option<PieceType>,
        castling: Option<CASTLE>,
    ) -> Move {
        assert_ne!(Some(PieceType::KING), promotion);
        assert_ne!(Some(PieceType::PAWN), promotion);

        Move {
            source,
            target,
            promotion,
            castling,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.source, self.target)
    }
}
