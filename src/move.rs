use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub source: SQUARE,
    pub target: SQUARE,
    pub promotion: Option<PieceType>,
    pub castling: Option<CASTLE>,
    pub capture: Option<PIECE>,
    pub en_passant: bool,
}

impl Move {
    pub fn new(
        source: SQUARE,
        target: SQUARE,
        promotion: Option<PieceType>,
        castling: Option<CASTLE>,
        capture: Option<PIECE>,
        en_passant: bool,
    ) -> Move {
        // can't promote to king or pawn
        assert_ne!(Some(PieceType::KING), promotion);
        assert_ne!(Some(PieceType::PAWN), promotion);

        Move {
            source,
            target,
            promotion,
            castling,
            capture,
            en_passant,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = format!("{}{}", self.source, self.target);
        write!(f, "{}", out.to_ascii_lowercase())?;
        if let Some(promotion) = self.promotion {
            let promotion = match promotion {
                PieceType::QUEEN => "q",
                PieceType::ROOK => "r",
                PieceType::BISHOP => "b",
                PieceType::KNIGHT => "n",
                _ => panic!("Invalid promotion"),
            };
            write!(f, "{}", promotion)?;
        }
        // write!(f, " (")?;
        // if let Some(castling) = self.castling {
        //     write!(f, " Castling: {}", castling)?;
        // }
        // if let Some(capture) = self.capture {
        //     write!(f, " Capture: {}", capture)?;
        // }
        // if self.en_passant {
        //     write!(f, " En Passant")?;
        // }
        // write!(f, ")")?;
        Ok(())
    }
}
