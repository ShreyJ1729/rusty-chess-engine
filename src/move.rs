use crate::enums::*;
use std::fmt::{Display, Formatter};

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
        };

        Ok(())
    }
}
