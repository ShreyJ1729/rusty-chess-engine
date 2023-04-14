use crate::*;

#[derive(Debug, Clone)]
pub struct Board {
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

    pub to_move: COLOR,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Board {
    pub fn new() -> Board {
        Self {
            to_move: COLOR::WHITE,
            castling_rights: CastlingRights::default(),
            halfmove_clock: 0,
            fullmove_number: 1,

            white_pawns: Bitboard::default(),
            white_knights: Bitboard::default(),
            white_bishops: Bitboard::default(),
            white_rooks: Bitboard::default(),
            white_queens: Bitboard::default(),
            white_king: Bitboard::default(),

            black_pawns: Bitboard::default(),
            black_knights: Bitboard::default(),
            black_bishops: Bitboard::default(),
            black_rooks: Bitboard::default(),
            black_queens: Bitboard::default(),
            black_king: Bitboard::default(),
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();
        // Reverse the order of ranks in the FEN string so that chars go from A1..=H8
        let fen: String = fen
            .split("/")
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .flat_map(|s: &str| s.chars())
            .collect();

        let mut index = 0;
        for c in fen.chars() {
            match c {
                'P' => board.white_pawns.set(index),
                'N' => board.white_knights.set(index),
                'B' => board.white_bishops.set(index),
                'R' => board.white_rooks.set(index),
                'Q' => board.white_queens.set(index),
                'K' => board.white_king.set(index),

                'p' => board.black_pawns.set(index),
                'n' => board.black_knights.set(index),
                'b' => board.black_bishops.set(index),
                'r' => board.black_rooks.set(index),
                'q' => board.black_queens.set(index),
                'k' => board.black_king.set(index),

                num => {
                    let num = num.to_digit(10).expect("Invalid FEN character");
                    assert!(num >= 1 && num <= 8, "Invalid FEN character");
                    index += num as usize - 1;
                }
            }
            index += 1;
        }

        board
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;
            let mut empty_squares = 0;
            for file in FILE::iter() {
                let file_index = file as usize;
                let square = index_to_bits(rank_index * 8 + file_index);
                let piece = self.piece_at(square);
                match piece.not_empty() {
                    true => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push(piece as u8 as char);
                    }
                    false => {
                        empty_squares += 1;
                    }
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank_index > 0 {
                fen.push('/');
            }
        }

        fen
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

    pub fn all_white_pieces(&self) -> Bitboard {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    pub fn all_black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn occupancy(&self) -> Bitboard {
        self.all_white_pieces() | self.all_black_pieces()
    }

    pub fn piece_at(&self, bits: u64) -> PIECE {
        let index = bits_to_index(bits);

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

    pub fn remove_piece(&mut self, bits: u64, piece: PIECE) {
        let index = bits_to_index(bits);

        match piece {
            PIECE::WhitePawn => self.white_pawns.unset(index),
            PIECE::WhiteKnight => self.white_knights.unset(index),
            PIECE::WhiteBishop => self.white_bishops.unset(index),
            PIECE::WhiteRook => self.white_rooks.unset(index),
            PIECE::WhiteQueen => self.white_queens.unset(index),
            PIECE::WhiteKing => self.white_king.unset(index),

            PIECE::BlackPawn => self.black_pawns.unset(index),
            PIECE::BlackKnight => self.black_knights.unset(index),
            PIECE::BlackBishop => self.black_bishops.unset(index),
            PIECE::BlackRook => self.black_rooks.unset(index),
            PIECE::BlackQueen => self.black_queens.unset(index),
            PIECE::BlackKing => self.black_king.unset(index),

            PIECE::Empty => panic!("Cannot remove empty piece"),
        }
    }

    pub fn add_piece(&mut self, bits: u64, piece: PIECE) {
        let index = bits_to_index(bits);

        match piece {
            PIECE::WhitePawn => self.white_pawns.set(index),
            PIECE::WhiteKnight => self.white_knights.set(index),
            PIECE::WhiteBishop => self.white_bishops.set(index),
            PIECE::WhiteRook => self.white_rooks.set(index),
            PIECE::WhiteQueen => self.white_queens.set(index),
            PIECE::WhiteKing => self.white_king.set(index),

            PIECE::BlackPawn => self.black_pawns.set(index),
            PIECE::BlackKnight => self.black_knights.set(index),
            PIECE::BlackBishop => self.black_bishops.set(index),
            PIECE::BlackRook => self.black_rooks.set(index),
            PIECE::BlackQueen => self.black_queens.set(index),
            PIECE::BlackKing => self.black_king.set(index),

            PIECE::Empty => panic!("Cannot add empty piece"),
        }
    }

    pub fn make_move(&mut self, move_: Move) {
        let target_piece = self.piece_at(move_.target);
        match target_piece.not_empty() {
            true => {
                self.remove_piece(move_.target, target_piece);
            }
            false => {}
        }

        let source_piece = self.piece_at(move_.source);
        match source_piece.not_empty() {
            true => {
                self.remove_piece(move_.source, source_piece);
                self.add_piece(move_.target, source_piece);
            }
            false => panic!("No piece at source square"),
        }

        // match move_.promotion {
        //     Some(new_piece) => {
        //         self.remove_piece(move_.target, new_piece);
        //         self.add_piece(move_.target, new_piece);
        //     }
        //     None => {}
        // }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // since bitboard is printed with rank 8 at the top, we need to iterate in reverse
        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;

            // writeln!(f, "{} at index {} ", rank, rank_index)?;

            write!(f, "{} ", rank_index + 1)?;

            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;

                let piece = self.piece_at(index_to_bits(index));

                let c = match piece.not_empty() {
                    true => piece as u8 as char,
                    false => ' ',
                };

                write!(f, "{}", c)?;
                write!(f, " ")?;
            }

            write!(f, "\n")?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        return Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }
}
