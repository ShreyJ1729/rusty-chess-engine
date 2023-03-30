use crate::*;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub piece: Option<PIECE>,
    pub color: Option<COLOR>,
}

impl Cell {
    pub fn new(piece: Option<PIECE>, color: Option<COLOR>) -> Self {
        Self {
            piece: piece,
            color: color,
        }
    }

    pub fn from_char(char: char) -> Self {
        match char {
            'r' => Cell::new(Some(PIECE::ROOK), Some(COLOR::BLACK)),
            'n' => Cell::new(Some(PIECE::KNIGHT), Some(COLOR::BLACK)),
            'b' => Cell::new(Some(PIECE::BISHOP), Some(COLOR::BLACK)),
            'q' => Cell::new(Some(PIECE::QUEEN), Some(COLOR::BLACK)),
            'k' => Cell::new(Some(PIECE::KING), Some(COLOR::BLACK)),
            'p' => Cell::new(Some(PIECE::PAWN), Some(COLOR::BLACK)),
            'R' => Cell::new(Some(PIECE::ROOK), Some(COLOR::WHITE)),
            'N' => Cell::new(Some(PIECE::KNIGHT), Some(COLOR::WHITE)),
            'B' => Cell::new(Some(PIECE::BISHOP), Some(COLOR::WHITE)),
            'Q' => Cell::new(Some(PIECE::QUEEN), Some(COLOR::WHITE)),
            'K' => Cell::new(Some(PIECE::KING), Some(COLOR::WHITE)),
            'P' => Cell::new(Some(PIECE::PAWN), Some(COLOR::WHITE)),
            _ => {
                panic!("Invalid character: {}", char);
            }
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(None, None)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.color {
            Some(COLOR::WHITE) => match self.piece {
                Some(PIECE::PAWN) => write!(f, "P"),
                Some(PIECE::KNIGHT) => write!(f, "N"),
                Some(PIECE::BISHOP) => write!(f, "B"),
                Some(PIECE::ROOK) => write!(f, "R"),
                Some(PIECE::QUEEN) => write!(f, "Q"),
                Some(PIECE::KING) => write!(f, "K"),
                None => write!(f, "."),
            },
            Some(COLOR::BLACK) => match self.piece {
                Some(PIECE::PAWN) => write!(f, "p"),
                Some(PIECE::KNIGHT) => write!(f, "n"),
                Some(PIECE::BISHOP) => write!(f, "b"),
                Some(PIECE::ROOK) => write!(f, "r"),
                Some(PIECE::QUEEN) => write!(f, "q"),
                Some(PIECE::KING) => write!(f, "k"),
                None => write!(f, "."),
            },
            None => write!(f, "."),
        }
    }
}
