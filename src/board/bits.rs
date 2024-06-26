use crate::bitboard::*;
use crate::board::Board;
use crate::enums::*;

impl<'a> Board<'a> {
    pub fn piece_at_square(&self, square: SQUARE) -> PIECE {
        self.piece_at_index(square.index())
    }

    pub fn piece_at_index(&self, index: usize) -> PIECE {
        if self.white_pawns.is_set(index) {
            PIECE::WhitePawn
        } else if self.white_knights.is_set(index) {
            PIECE::WhiteKnight
        } else if self.white_bishops.is_set(index) {
            PIECE::WhiteBishop
        } else if self.white_rooks.is_set(index) {
            PIECE::WhiteRook
        } else if self.white_queens.is_set(index) {
            PIECE::WhiteQueen
        } else if self.white_king.is_set(index) {
            PIECE::WhiteKing
        } else if self.black_pawns.is_set(index) {
            PIECE::BlackPawn
        } else if self.black_knights.is_set(index) {
            PIECE::BlackKnight
        } else if self.black_bishops.is_set(index) {
            PIECE::BlackBishop
        } else if self.black_rooks.is_set(index) {
            PIECE::BlackRook
        } else if self.black_queens.is_set(index) {
            PIECE::BlackQueen
        } else if self.black_king.is_set(index) {
            PIECE::BlackKing
        } else {
            PIECE::Empty
        }
    }

    pub fn clear(&mut self) {
        self.white_pawns.clear();
        self.white_knights.clear();
        self.white_bishops.clear();
        self.white_rooks.clear();
        self.white_queens.clear();
        self.white_king.clear();

        self.black_pawns.clear();
        self.black_knights.clear();
        self.black_bishops.clear();
        self.black_rooks.clear();
        self.black_queens.clear();
        self.black_king.clear();
    }

    pub fn white_occupancy(&self) -> Bitboard {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    pub fn black_occupancy(&self) -> Bitboard {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn occupancy(&self) -> Bitboard {
        self.white_occupancy() | self.black_occupancy()
    }

    pub fn occupancy_of_color(&self, color: COLOR) -> Bitboard {
        match color {
            COLOR::WHITE => self.white_occupancy(),
            COLOR::BLACK => self.black_occupancy(),
        }
    }

    pub fn occupancy_of_piece(&self, piece: PIECE) -> Bitboard {
        match piece {
            PIECE::WhitePawn => self.white_pawns,
            PIECE::WhiteKnight => self.white_knights,
            PIECE::WhiteBishop => self.white_bishops,
            PIECE::WhiteRook => self.white_rooks,
            PIECE::WhiteQueen => self.white_queens,
            PIECE::WhiteKing => self.white_king,

            PIECE::BlackPawn => self.black_pawns,
            PIECE::BlackKnight => self.black_knights,
            PIECE::BlackBishop => self.black_bishops,
            PIECE::BlackRook => self.black_rooks,
            PIECE::BlackQueen => self.black_queens,
            PIECE::BlackKing => self.black_king,

            PIECE::Empty => !self.occupancy(),
        }
    }
}
