use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub to_move: COLOR,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub en_passant_target: Option<SQUARE>,

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
    pub move_history: Vec<Move>,
}

impl<'a> Board<'a> {
    pub fn new(lookup_table: &'a LookupTable) -> Board<'a> {
        Self {
            to_move: COLOR::WHITE,
            castling_rights: CastlingRights::default(),
            halfmove_clock: 0,
            fullmove_number: 1,

            en_passant_target: None,

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
            move_history: Vec::new(),
        }
    }

    // ---------------------------------------------
    // --------------- FEN NOTATION ----------------
    // ---------------------------------------------

    pub fn from_fen(fen: &str, lookup_table: &'a LookupTable) -> Board<'a> {
        let mut board = Board::new(lookup_table);

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

        // adding board data
        for rank in RANK::iter().rev() {
            let rank_index = rank as usize;
            let mut empty_squares = 0;
            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;
                let piece = self.piece_at_index(index);
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

        // adding turn
        fen.push(' ');
        fen.push(self.to_move.to_fen());

        // adding castling rights
        fen.push(' ');
        fen.push_str(&self.castling_rights.to_fen());

        // adding en passant target
        fen.push(' ');
        match self.en_passant_target {
            Some(square) => fen.push_str(&square.to_fen()),
            None => fen.push('-'),
        };

        // adding halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // adding fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    pub fn starting_position(lookup_table: &LookupTable) -> Board {
        Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            lookup_table,
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

        // now for each source square index (each piece of that type on the board, or each bit on the bitboard)
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
            let target_squares = move_bb.get_squares();

            // 3. build the moves and push to the moves vector
            for target_square in target_squares {
                let is_pawn_promotion =
                    piece_type == PieceType::PAWN && target_square.is_pawn_promote(color);

                let is_en_passant = piece_type == PieceType::PAWN
                    && match color {
                        COLOR::WHITE => {
                            Some(target_square) == self.en_passant_target
                                && source_square.rank() == RANK::Rank5
                                && target_square.rank() == RANK::Rank6
                        }
                        COLOR::BLACK => {
                            Some(target_square) == self.en_passant_target
                                && source_square.rank() == RANK::Rank4
                                && target_square.rank() == RANK::Rank3
                        }
                    };

                let capture = match self.piece_at(target_square) {
                    PIECE::Empty => None,
                    piece => Some(piece),
                };

                // if it's pawn promotion, add four moves (one for each promotion piece)
                match is_pawn_promotion {
                    true => {
                        for promotion_option in PROMOTION_OPTIONS.iter() {
                            moves.push(Move::new(
                                source_square,
                                target_square,
                                Some(*promotion_option),
                                None,
                                capture,
                                false, // if pawn promotion, en passant is not possible
                            ));
                        }
                    }
                    // if not, just add the move
                    false => {
                        moves.push(Move::new(
                            source_square,
                            target_square,
                            None,
                            None,
                            capture,
                            is_en_passant,
                        ));
                    }
                }
            }

            // special case - if it's a king move, add castling moves if castling rights
            if piece_type == PieceType::KING {
                match color {
                    COLOR::WHITE => {
                        if self.castling_rights.get(CASTLE::WhiteKingside) {
                            moves.push(Move::new(
                                source_square,
                                SQUARE::G1,
                                None,
                                Some(CASTLE::WhiteKingside),
                                None,
                                false,
                            ));
                        }
                        if self.castling_rights.get(CASTLE::WhiteQueenside) {
                            moves.push(Move::new(
                                source_square,
                                SQUARE::C1,
                                None,
                                Some(CASTLE::WhiteQueenside),
                                None,
                                false,
                            ));
                        }
                    }
                    COLOR::BLACK => {
                        if self.castling_rights.get(CASTLE::BlackKingside) {
                            moves.push(Move::new(
                                source_square,
                                SQUARE::G8,
                                None,
                                Some(CASTLE::BlackKingside),
                                None,
                                false,
                            ));
                        }
                        if self.castling_rights.get(CASTLE::BlackQueenside) {
                            moves.push(Move::new(
                                source_square,
                                SQUARE::C8,
                                None,
                                Some(CASTLE::BlackQueenside),
                                None,
                                false,
                            ));
                        }
                    }
                }
            }
        }

        // finally we filter out invalid moves
        MoveValidator::filter_valid_moves(&self, &mut moves);

        moves
    }

    pub fn generate_moves_for_square(&self, square: SQUARE) -> Vec<Move> {
        let piece = self.piece_at(square);
        let bb = Bitboard::new(square.bits());
        println!(
            "generating moves for square {} with piece {}",
            square, piece,
        );
        println!("bb: {}", bb);
        self.generate_moves_for_piece(piece)
    }

    pub fn generate_moves_for_color(&self, color: COLOR) -> Vec<Move> {
        vec![
            self.generate_moves_for_piece(PieceType::PAWN.for_color(color)),
            self.generate_moves_for_piece(PieceType::KNIGHT.for_color(color)),
            self.generate_moves_for_piece(PieceType::BISHOP.for_color(color)),
            self.generate_moves_for_piece(PieceType::ROOK.for_color(color)),
            self.generate_moves_for_piece(PieceType::QUEEN.for_color(color)),
            self.generate_moves_for_piece(PieceType::KING.for_color(color)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    // ---------------------------------------------
    // -------------- PIECE MOVEMENT ---------------
    // ---------------------------------------------

    pub fn piece_at(&self, square: SQUARE) -> PIECE {
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

    pub fn remove_piece(&mut self, index: usize) {
        let piece = self.piece_at_index(index);
        match piece {
            PIECE::WhitePawn => self.white_pawns.unset(index),
            PIECE::WhiteKnight => self.white_knights.unset(index),
            PIECE::WhiteBishop => self.white_bishops.unset(index),
            PIECE::WhiteRook => {
                self.white_rooks.unset(index);
                match SQUARE::from(index) {
                    SQUARE::A1 => self.castling_rights.set(CASTLE::WhiteQueenside, false),
                    SQUARE::H1 => self.castling_rights.set(CASTLE::WhiteKingside, false),
                    _ => {}
                }
            }
            PIECE::WhiteQueen => self.white_queens.unset(index),

            PIECE::BlackPawn => self.black_pawns.unset(index),
            PIECE::BlackKnight => self.black_knights.unset(index),
            PIECE::BlackBishop => self.black_bishops.unset(index),
            PIECE::BlackRook => {
                self.black_rooks.unset(index);
                match SQUARE::from(index) {
                    SQUARE::A8 => self.castling_rights.set(CASTLE::BlackQueenside, false),
                    SQUARE::H8 => self.castling_rights.set(CASTLE::BlackKingside, false),
                    _ => {}
                }
            }
            PIECE::BlackQueen => self.black_queens.unset(index),

            // temporarily remove the king from the board, will check for endgame conditions later
            PIECE::WhiteKing => {
                self.white_king.unset(index);
            }

            PIECE::BlackKing => {
                self.black_king.unset(index);
            }

            PIECE::Empty => {
                println!("board:\n{}", self);
                panic!(
                    "Tried to remove empty piece at square {}",
                    SQUARE::from(index)
                );
            }
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

            PIECE::Empty => {
                println!(
                    "Tried to add empty piece at square {} on below board",
                    SQUARE::from(index)
                );
                println!("{}", self);
                panic!("Cannot add empty piece");
            }
        }
    }

    pub fn make_move(&mut self, move_: Move) {
        self.move_history.push(move_);
        let source_square = move_.source;
        let target_square = move_.target;
        let target_index = move_.target.index();
        let source_index = move_.source.index();
        let source_piece = self.piece_at_index(source_index);
        let source_color = match source_piece.color() {
            Some(COLOR::WHITE) => COLOR::WHITE,
            Some(COLOR::BLACK) => COLOR::BLACK,
            None => {
                println!("move: {}", move_);
                panic!("Cannot make move from empty square");
            }
        };

        // clear the recently removed flag from last turn
        self.castling_rights.clear_recently_removed(source_color);

        // updating halfmove clock for capture
        if self.piece_at_index(target_index).not_empty() {
            self.halfmove_clock = 0;
        }

        // reset en passant target if it was set
        if self.en_passant_target.is_some() {
            self.en_passant_target = None;
        }

        // updating halfmove clock for pawn move and add en passant square if double pawn move
        // en passant target is the square behind the pawn that can be captured
        if source_piece.piece_type() == PieceType::PAWN {
            self.halfmove_clock = 0;
            match source_color {
                COLOR::WHITE => {
                    if source_square.rank() == RANK::Rank2 && move_.target.rank() == RANK::Rank4 {
                        let en_passant_target = bits_to_index(
                            north(source_square.bits())
                                .expect("Pawn double move cannot be on rank 8"),
                        );
                        self.en_passant_target = Some(SQUARE::from(en_passant_target));
                    }
                }
                COLOR::BLACK => {
                    if source_square.rank() == RANK::Rank7 && move_.target.rank() == RANK::Rank5 {
                        let en_passant_target = bits_to_index(
                            south(source_square.bits())
                                .expect("Pawn double move cannot be on rank 1"),
                        );
                        self.en_passant_target = Some(SQUARE::from(en_passant_target));
                    }
                }
            }
        }

        // perform castling move and return if castling:
        match move_.castling {
            Some(CASTLE::WhiteKingside) => {
                self.remove_piece(SQUARE::E1.index());
                self.remove_piece(SQUARE::H1.index());
                self.add_piece(SQUARE::G1.index(), PIECE::WhiteKing);
                self.add_piece(SQUARE::F1.index(), PIECE::WhiteRook);
                self.castling_rights.remove_color(COLOR::WHITE);
            }
            Some(CASTLE::WhiteQueenside) => {
                self.remove_piece(SQUARE::E1.index());
                self.remove_piece(SQUARE::A1.index());
                self.add_piece(SQUARE::C1.index(), PIECE::WhiteKing);
                self.add_piece(SQUARE::D1.index(), PIECE::WhiteRook);
                self.castling_rights.remove_color(COLOR::WHITE);
            }
            Some(CASTLE::BlackKingside) => {
                self.remove_piece(SQUARE::E8.index());
                self.remove_piece(SQUARE::H8.index());
                self.add_piece(SQUARE::G8.index(), PIECE::BlackKing);
                self.add_piece(SQUARE::F8.index(), PIECE::BlackRook);
                self.castling_rights.remove_color(COLOR::BLACK);
            }
            Some(CASTLE::BlackQueenside) => {
                self.remove_piece(SQUARE::E8.index());
                self.remove_piece(SQUARE::A8.index());
                self.add_piece(SQUARE::C8.index(), PIECE::BlackKing);
                self.add_piece(SQUARE::D8.index(), PIECE::BlackRook);
                self.castling_rights.remove_color(COLOR::BLACK);
            }

            // if not castling make move as normal
            None => {
                // remove target for captures
                if self.piece_at_index(target_index).not_empty() {
                    self.remove_piece(target_index);
                }
                // move piece
                match source_piece.not_empty() {
                    true => {
                        self.remove_piece(source_index);
                        self.add_piece(target_index, source_piece);
                    }
                    false => panic!("No piece at source square"),
                }

                // handle promotion
                match move_.promotion {
                    Some(new_piece) => {
                        self.remove_piece(target_index);
                        self.add_piece(target_index, new_piece.for_color(source_color));
                    }
                    None => {}
                }

                // handle en passant capture
                if move_.en_passant {
                    // depending on color of moving piece, remove the piece one rank above or below the target square
                    match source_color {
                        COLOR::WHITE => {
                            let to_remove_idx = bits_to_index(
                                south(target_square.bits())
                                    .expect("En passant cannot be on rank 1"),
                            );
                            if self.piece_at_index(to_remove_idx) != PIECE::BlackPawn {
                                panic!("En passant capture not on black pawn");
                            }
                            self.remove_piece(to_remove_idx)
                        }
                        COLOR::BLACK => {
                            let to_remove_idx = bits_to_index(
                                north(target_square.bits())
                                    .expect("En passant cannot be on rank 8"),
                            );
                            if self.piece_at_index(to_remove_idx) != PIECE::WhitePawn {
                                panic!("En passant capture not on white pawn");
                            }
                            self.remove_piece(to_remove_idx)
                        }
                    }
                }

                // updating castling rights for non-castle king moves
                if source_piece.piece_type() == PieceType::KING {
                    self.castling_rights.remove_color(source_color);
                }

                // updating castling rights for non-castle rook moves
                if source_piece.piece_type() == PieceType::ROOK {
                    match source_square {
                        SQUARE::A1 => self.castling_rights.set(CASTLE::WhiteQueenside, false),
                        SQUARE::H1 => self.castling_rights.set(CASTLE::WhiteKingside, false),
                        SQUARE::A8 => self.castling_rights.set(CASTLE::BlackQueenside, false),
                        SQUARE::H8 => self.castling_rights.set(CASTLE::BlackKingside, false),
                        _ => {}
                    }
                }
            }
        }

        // update clocks
        self.halfmove_clock += 1;
        if source_color == COLOR::BLACK {
            self.fullmove_number += 1;
        }

        // change to_move
        self.to_move = self.to_move.opposite();

        // check if board in a state where A1 is empty, but castling rights on wq are still set
        if self.castling_rights.get(CASTLE::WhiteQueenside) {
            if self.piece_at(SQUARE::A1) != PIECE::WhiteRook {
                panic!(
                    "White queenside castling rights set but no rook on A1! Board:\n{}",
                    self
                );
            }
        }
    }

    // This is a special function that allows us to make any move specified by the Move struct
    // This exists so that we can easily unmake moves when we are checking if the king is in check and for recursive perft testing
    // That way we don't have to make a board copy every time we want to check if a move is valid or when we do a layer of recursion
    pub fn unmake_move(&mut self, _move: Move) {
        self.move_history.pop();
        let source = _move.source;
        let target = _move.target;
        let source_piece = self.piece_at(source);
        let target_piece = self.piece_at(target);
        let just_moved_color = target_piece.color().expect("Invalid source piece");
        let captured_piece = _move.capture;
        let promotion = _move.promotion;
        let en_passant = _move.en_passant;
        let castling = _move.castling;

        // move the piece back
        self.add_piece(source.index(), target_piece);
        self.remove_piece(target.index());

        // if non-enpassant capture, add the captured piece back to the board
        if captured_piece.is_some() && !en_passant {
            self.add_piece(target.index(), captured_piece.unwrap());
        }

        // if en passant, add the captured pawn back to the board
        if en_passant {
            match just_moved_color {
                COLOR::WHITE => {
                    let en_passant_target = bits_to_index(
                        south(target.bits()).expect("En passant cannot be on rank 1"),
                    );
                    self.add_piece(en_passant_target, PIECE::BlackPawn);
                }
                COLOR::BLACK => {
                    let en_passant_target = bits_to_index(
                        north(target.bits()).expect("En passant cannot be on rank 8"),
                    );
                    self.add_piece(en_passant_target, PIECE::WhitePawn);
                }
            }
        }

        // if promotion, demote the piece back to a pawn
        if promotion.is_some() {
            self.remove_piece(source.index());
            self.add_piece(source.index(), PieceType::PAWN.for_color(just_moved_color));
        }

        // if castling, move the rook back
        match castling {
            Some(CASTLE::WhiteKingside) => {
                self.remove_piece(SQUARE::F1.index());
                self.add_piece(SQUARE::H1.index(), PIECE::WhiteRook);
            }
            Some(CASTLE::WhiteQueenside) => {
                self.remove_piece(SQUARE::D1.index());
                self.add_piece(SQUARE::A1.index(), PIECE::WhiteRook);
            }
            Some(CASTLE::BlackKingside) => {
                self.remove_piece(SQUARE::F8.index());
                self.add_piece(SQUARE::H8.index(), PIECE::BlackRook);
            }
            Some(CASTLE::BlackQueenside) => {
                self.remove_piece(SQUARE::D8.index());
                self.add_piece(SQUARE::A8.index(), PIECE::BlackRook);
            }
            None => {}
        }

        // give castling rights if castling was undone
        // We have to be careful to only give back the rights that were recently removed
        // This is because it could be the case that one rook moved somewhere and then back to orig square
        if castling.is_some() {
            match just_moved_color {
                COLOR::WHITE => {
                    if self.castling_rights.recently_removed[CASTLE::WhiteKingside as usize] {
                        self.castling_rights.set(CASTLE::WhiteKingside, true);
                    }
                    if self.castling_rights.recently_removed[CASTLE::WhiteQueenside as usize] {
                        self.castling_rights.set(CASTLE::WhiteQueenside, true);
                    }
                }
                COLOR::BLACK => {
                    if self.castling_rights.recently_removed[CASTLE::BlackKingside as usize] {
                        self.castling_rights.set(CASTLE::BlackKingside, true);
                    }
                    if self.castling_rights.recently_removed[CASTLE::BlackQueenside as usize] {
                        self.castling_rights.set(CASTLE::BlackQueenside, true);
                    }
                }
            }
        }

        // If one or more of our castlingrights were recently removed, give them back
        // Then set the recently removed flag for current color to false
        match just_moved_color {
            COLOR::WHITE => {
                for i in 0..=1 {
                    if self.castling_rights.recently_removed[i] {
                        self.castling_rights.set_index(i, true);
                    }
                }
            }
            COLOR::BLACK => {
                for i in 2..=3 {
                    if self.castling_rights.recently_removed[i] {
                        self.castling_rights.set_index(i, true);
                    }
                }
            }
        }

        // if pawn was not moved or piece was not captured, increment halfmove clock
        if source_piece.piece_type() != PieceType::PAWN
            || captured_piece.is_none() && self.halfmove_clock > 0
        {
            self.halfmove_clock -= 1;
        }

        if self.to_move == COLOR::WHITE {
            self.fullmove_number -= 1;
        }

        self.to_move = self.to_move.opposite();

        // check if board in a state where A1 is empty, but castling rights on wq are still set
        if self.castling_rights.get(CASTLE::WhiteQueenside) {
            if self.piece_at(SQUARE::A1) != PIECE::WhiteRook {
                panic!(
                    "UNMAKED White queenside castling rights set but no rook on A1! Board:\n{}. Unmake move: {}",
                    self, _move
                );
            }
        }
    }

    // ---------------------------------------------
    // ------------------ PERFT --------------------
    // ---------------------------------------------

    pub fn perft(
        &mut self,
        depth: u8,
        max_depth: u8,
        move_counter: &mut HashMap<String, u64>,
    ) -> (u64, u64, u64, u64, u64, u64) {
        if depth == 0 {
            return (1, 0, 0, 0, 0, 0);
        }

        let mut nodes = 0;
        let mut captures = 0;
        let mut castles = 0;
        let mut en_passants = 0;
        let mut promotions = 0;
        let mut checks = 0;
        let mut checkmates = 0;

        let moves = self.generate_moves_for_color(self.to_move);

        // enumerate m and idx for moves
        for (i, m) in moves.iter().enumerate() {
            if m.capture.is_some() {
                captures += 1;
            }
            if m.castling.is_some() {
                castles += 1;
            }
            if m.en_passant {
                en_passants += 1;
            }
            if m.promotion.is_some() {
                promotions += 1;
            }

            // self.make_move(*m);
            let mut board = self.clone();
            board.make_move(*m);
            if MoveValidator::in_check(&board, COLOR::WHITE)
                || MoveValidator::in_check(&board, COLOR::BLACK)
            {
                checks += 1;
            }
            let (n, c, ca, en, pro, ch) = board.perft(depth - 1, max_depth, move_counter);
            // self.unmake_move(*m);
            nodes += n;
            captures += c;
            castles += ca;
            en_passants += en;
            promotions += pro;
            checks += ch;

            if depth == max_depth {
                println!("({}/{}) {}: {}", i + 1, moves.len(), m, n);
                move_counter.insert(m.to_string(), n);
            }
        }
        (nodes, captures, castles, en_passants, promotions, checks)
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

            write!(f, "{} ", rank_index + 1)?;

            for file in FILE::iter() {
                let file_index = file as usize;
                let index = rank_index * 8 + file_index;

                let piece = self.piece_at_index(index);

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

        // print move history
        write!(f, "Move history: ")?;
        for (m) in self.move_history.iter() {
            write!(f, "{} ", m)?;
        }

        Ok(())
    }
}
