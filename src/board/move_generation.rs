use crate::board::Board;
use crate::enums::*;
use crate::r#move::Move;

impl<'a> Board<'a> {
    pub fn generate_moves_for_piece(&self, piece: PIECE) -> Vec<Move> {
        let mut moves = Vec::new();

        // get occupancy for sliding pieces
        let occupancy = self.occupancy().bits();

        // get the corresponding bitboard for the piece
        let mut piece_bb = piece.piece_bb(&self);

        let color = piece
            .color()
            .expect("Cannot generate moves for empty square");
        let piece_type = piece.piece_type();

        // now for each source square index (each piece of that type on the board, or each bit on the bitboard)
        while let Some(source_idx) = piece_bb.pop_lsb() {
            let source_square = SQUARE::from(source_idx);

            // 1. get the move bitboard (bb of target squares) for the piece at square
            let mut move_bb = match piece_type {
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

            // 3. Iterate over the target squares from the move bitboard
            while let Some(target_idx) = move_bb.pop_lsb() {
                let target_square = SQUARE::from(target_idx);
                let pawn_move = piece_type == PieceType::PAWN;
                let is_pawn_promotion = pawn_move && target_square.is_pawn_promote(color);

                let is_en_passant = pawn_move
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
                    target_piece => {
                        if target_piece.color() == Some(color.opposite()) {
                            Some(target_piece)
                        } else {
                            None
                        }
                    }
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
        self.filter_valid_moves(&mut moves);

        moves
    }

    pub fn generate_moves_for_square(&self, square: SQUARE) -> Vec<Move> {
        let piece = self.piece_at(square);
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
}
