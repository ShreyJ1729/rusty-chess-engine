use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::enums::piece::PieceType;
use crate::enums::COLOR;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum PIECE {
    WhitePawn = 80,   // P
    WhiteKnight = 78, // N
    WhiteBishop = 66, // B
    WhiteRook = 82,   // R
    WhiteQueen = 81,  // Q
    WhiteKing = 75,   // K

    BlackPawn = 112,   // p
    BlackKnight = 110, // n
    BlackBishop = 98,  // b
    BlackRook = 114,   // r
    BlackQueen = 113,  // q
    BlackKing = 107,   // k

    Empty = 32, // ' '
}

impl PIECE {
    pub fn color(&self) -> Option<COLOR> {
        match self {
            PIECE::WhitePawn
            | PIECE::WhiteKnight
            | PIECE::WhiteBishop
            | PIECE::WhiteRook
            | PIECE::WhiteQueen
            | PIECE::WhiteKing => Some(COLOR::WHITE),
            PIECE::BlackPawn
            | PIECE::BlackKnight
            | PIECE::BlackBishop
            | PIECE::BlackRook
            | PIECE::BlackQueen
            | PIECE::BlackKing => Some(COLOR::BLACK),
            PIECE::Empty => None,
        }
    }

    // given a board, returns the bitboard of all pieces of this type
    pub fn piece_bb(&self, board: &Board) -> Bitboard {
        match self {
            PIECE::WhitePawn => board.white_pawns,
            PIECE::WhiteKnight => board.white_knights,
            PIECE::WhiteBishop => board.white_bishops,
            PIECE::WhiteRook => board.white_rooks,
            PIECE::WhiteQueen => board.white_queens,
            PIECE::WhiteKing => board.white_king,

            PIECE::BlackPawn => board.black_pawns,
            PIECE::BlackKnight => board.black_knights,
            PIECE::BlackBishop => board.black_bishops,
            PIECE::BlackRook => board.black_rooks,
            PIECE::BlackQueen => board.black_queens,
            PIECE::BlackKing => board.black_king,

            PIECE::Empty => Bitboard::default(),
        }
    }

    pub fn is_white(&self) -> bool {
        self.color() == Some(COLOR::WHITE)
    }

    pub fn is_black(&self) -> bool {
        self.color() == Some(COLOR::BLACK)
    }

    pub fn is_color(&self, color: COLOR) -> bool {
        self.not_empty() && self.color() == Some(color)
    }

    pub fn is_opposite_color(&self, color: COLOR) -> bool {
        self.not_empty() && self.color() != Some(color)
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            PIECE::WhitePawn | PIECE::BlackPawn => true,
            _ => false,
        }
    }

    pub fn is_knight(&self) -> bool {
        match self {
            PIECE::WhiteKnight | PIECE::BlackKnight => true,
            _ => false,
        }
    }

    pub fn is_bishop(&self) -> bool {
        match self {
            PIECE::WhiteBishop | PIECE::BlackBishop => true,
            _ => false,
        }
    }

    pub fn is_rook(&self) -> bool {
        match self {
            PIECE::WhiteRook | PIECE::BlackRook => true,
            _ => false,
        }
    }

    pub fn is_queen(&self) -> bool {
        match self {
            PIECE::WhiteQueen | PIECE::BlackQueen => true,
            _ => false,
        }
    }

    pub fn is_king(&self) -> bool {
        match self {
            PIECE::WhiteKing | PIECE::BlackKing => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PIECE::Empty => true,
            _ => false,
        }
    }

    pub fn not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            PIECE::WhitePawn | PIECE::BlackPawn => PieceType::PAWN,
            PIECE::WhiteKnight | PIECE::BlackKnight => PieceType::KNIGHT,
            PIECE::WhiteBishop | PIECE::BlackBishop => PieceType::BISHOP,
            PIECE::WhiteRook | PIECE::BlackRook => PieceType::ROOK,
            PIECE::WhiteQueen | PIECE::BlackQueen => PieceType::QUEEN,
            PIECE::WhiteKing | PIECE::BlackKing => PieceType::KING,
            PIECE::Empty => PieceType::EMPTY,
        }
    }
}
