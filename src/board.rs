use crate::*;

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub to_move: COLOR,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub en_passant_available: bool,
    pub en_passant_target: Option<SQUARE>,
    pub move_history: Option<Vec<Move>>,

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

    pub lookup_table: &'a LookupTable,
    pub move_validator: MoveValidator,
}

impl<'a> Board<'a> {
    pub fn new(lookup_table: &'a LookupTable, keep_move_history: bool) -> Board<'a> {
        Self {
            to_move: COLOR::WHITE,
            castling_rights: CastlingRights::default(),
            halfmove_clock: 0,
            fullmove_number: 1,

            en_passant_available: false,
            en_passant_target: None,
            move_history: match keep_move_history {
                true => Some(Vec::new()),
                false => None,
            },

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

            lookup_table,
            move_validator: MoveValidator::new(),
        }
    }

    // ---------------------------------------------
    // --------------- FEN NOTATION ----------------
    // ---------------------------------------------

    pub fn from_fen(
        fen: &str,
        lookup_table: &'a LookupTable,
        keep_move_history: bool,
    ) -> Board<'a> {
        let mut board = Board::new(lookup_table, keep_move_history);

        // split the board configuration from metadata
        let fen = fen.split(" ").into_iter().collect::<Vec<&str>>();
        assert_eq!(fen.len(), 6, "Invalid FEN string: {}", fen.join(" "));

        let board_data = fen[0];
        let turn = fen[1];
        let castling_rights = fen[2];
        let en_passant_target = fen[3];
        let halfmove_clock = fen[4];
        let fullmove_number = fen[5];

        board.to_move = match turn {
            "w" => COLOR::WHITE,
            "b" => COLOR::BLACK,
            _ => panic!("Invalid turn: {}", turn),
        };

        board.castling_rights = CastlingRights::from_fen(castling_rights);

        board.en_passant_target = match en_passant_target {
            "-" => None,
            _ => SQUARE::from_string(en_passant_target),
        };

        board.halfmove_clock = halfmove_clock.parse().expect("Invalid halfmove clock");
        board.fullmove_number = fullmove_number.parse().expect("Invalid fullmove number");

        // Reverse the order of ranks in the FEN string so that chars go from A1..=H8
        let board_data: String = board_data
            .split("/")
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .flat_map(|s: &str| s.chars())
            .collect();

        let mut index = 0;
        for c in board_data.chars() {
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
                let square = rank_index * 8 + file_index;
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

    pub fn starting_position(lookup_table: &LookupTable) -> Board {
        Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            lookup_table,
            true,
        )
    }

    // ---------------------------------------------
    // ------------ BITS & OCCUPANCY ---------------
    // ---------------------------------------------

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

    pub fn pieces_of_color(&self, color: COLOR) -> Bitboard {
        match color {
            COLOR::WHITE => self.white_occupancy(),
            COLOR::BLACK => self.black_occupancy(),
        }
    }

    pub fn pieces_of_type(&self, piece_type: PieceType) -> Bitboard {
        match piece_type {
            PieceType::PAWN => self.white_pawns | self.black_pawns,
            PieceType::KNIGHT => self.white_knights | self.black_knights,
            PieceType::BISHOP => self.white_bishops | self.black_bishops,
            PieceType::ROOK => self.white_rooks | self.black_rooks,
            PieceType::QUEEN => self.white_queens | self.black_queens,
            PieceType::KING => self.white_king | self.black_king,
            PieceType::EMPTY => !self.occupancy(),
        }
    }

    pub fn pieces_of_color_and_type(&self, color: COLOR, piece_type: PieceType) -> Bitboard {
        self.pieces_of_color(color) & self.pieces_of_type(piece_type)
    }

    // ---------------------------------------------
    // ------------- MOVE GENERATION ---------------
    // ---------------------------------------------

    pub fn generate_moves_for_piece(&self, piece: PIECE) -> Vec<Move> {
        let mut moves = Vec::new();

        // get occupancy for sliding pieces
        let occupancy = self.occupancy().bits();

        // get the corresponding bitboard for the piece
        let piece_bb = piece.piece_bb(&self);
        let color = piece
            .color()
            .expect("Cannot generate moves for empty square");
        let piece_type = piece.piece_type();

        // get the indices of the bits in the bitboard (these are the source squares)
        let sources = piece_bb.indices();

        // now for each source square index
        for source in sources {
            let source_square = SQUARE::from(source);

            // 1. get the move bitboard (bb of target squares) for the piece at square
            let move_bb = match piece_type {
                PieceType::PAWN => self.lookup_table.get_pawn_moves(source_square, color),
                PieceType::KNIGHT => self.lookup_table.get_knight_moves(source_square, color),
                PieceType::BISHOP => {
                    self.lookup_table
                        .get_bishop_moves(source_square, color, occupancy)
                }
                PieceType::ROOK => {
                    self.lookup_table
                        .get_rook_moves(source_square, color, occupancy)
                }
                PieceType::QUEEN => {
                    self.lookup_table
                        .get_queen_moves(source_square, color, occupancy)
                }
                PieceType::KING => self.lookup_table.get_king_moves(source_square, color),
                PieceType::EMPTY => panic!("Cannot generate moves for empty square"),
            };

            // 2. get the indices of the bits in the move_bb (these are the target move squares)
            let target_indices = move_bb.indices();

            // 3. build the moves and push to the moves vector
            for target in target_indices {
                let move_ = Move::new(SQUARE::from(source), SQUARE::from(target), None);
                moves.push(move_);
            }
        }

        // finally we filter out invalid moves
        self.move_validator.filter_valid_moves(&self, &mut moves);

        moves
    }

    pub fn generate_moves_for_square(&self, square: SQUARE) -> Vec<Move> {
        let piece = self.piece_at(square.index());
        self.generate_moves_for_piece(piece)
    }

    pub fn generate_moves_for_color(&self, color: COLOR) -> Vec<Move> {
        vec![
            self.generate_moves_for_piece(PieceType::PAWN.for_color(color)),
            self.generate_moves_for_piece(PieceType::KNIGHT.for_color(color)),
            self.generate_moves_for_piece(PieceType::BISHOP.for_color(color)),
            self.generate_moves_for_piece(PieceType::ROOK.for_color(color)),
            self.generate_moves_for_piece(PieceType::QUEEN.for_color(color)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    // ---------------------------------------------
    // -------------- PIECE MOVEMENT ---------------
    // ---------------------------------------------

    pub fn piece_at(&self, index: usize) -> PIECE {
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

    pub fn remove_piece(&mut self, index: usize) {
        let piece = self.piece_at(index);
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

    pub fn add_piece(&mut self, index: usize, piece: PIECE) {
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
        let target_index = move_.target.index();
        if self.piece_at(target_index).not_empty() {
            self.halfmove_clock = 0;
            self.remove_piece(target_index);
        }

        let source_index = move_.source.index();
        let source_piece = self.piece_at(source_index);
        let source_color = source_piece.color().expect("Cannot move empty piece");
        if source_piece.piece_type() == PieceType::PAWN {
            self.halfmove_clock = 0;
        }
        match source_piece.not_empty() {
            true => {
                self.remove_piece(source_index);
                self.add_piece(target_index, source_piece);
            }
            false => panic!("No piece at source square"),
        }

        match move_.promotion {
            Some(new_piece) => {
                self.remove_piece(target_index);
                self.add_piece(target_index, new_piece.for_color(source_color));
            }
            None => {}
        }

        // if we're keeping track of the move history, add this move to it
        if let Some(move_history) = &mut self.move_history {
            move_history.push(move_);
        }
    }
}

// ---------------------------------------------
// ------------------ IMPLS --------------------
// ---------------------------------------------

impl Display for Board<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // since bitboard is printed with rank 8 at the top, we need to iterate in reverse
        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;

            // writeln!(f, "{} at index {} ", rank, rank_index)?;

            write!(f, "{} ", rank_index + 1)?;

            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;

                let piece = self.piece_at(index);

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
