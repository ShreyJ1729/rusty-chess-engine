use crate::board::Board;
use crate::enums::*;
use crate::r#move::Move;

impl<'a> Board<'a> {
    fn generate_moves_for_piece(&self, piece: PIECE) -> Vec<Move> {
        let mut moves = Vec::new();

        // get occupancy for sliding pieces
        let occupancy = self.occupancy().bits();

        // get the corresponding bitboard for the piece
        let mut piece_bb = piece.piece_bb(&self);

        let color = piece.color().unwrap();
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

            // 2. Iterate over the target squares from the move bitboard
            while let Some(target_idx) = move_bb.pop_lsb() {
                let target_square = SQUARE::from(target_idx);
                let is_pawn_promotion =
                    piece_type == PieceType::PAWN && target_square.is_pawn_promote(color);
                let mut is_en_passant = false;

                let capture = match self.piece_at_square(target_square) {
                    PIECE::Empty => None,
                    target_piece => {
                        if target_piece.color() == Some(color.opposite()) {
                            Some(target_piece)
                        } else {
                            None
                        }
                    }
                };

                // 4. If pawn, do some special checks
                if piece_type == PieceType::PAWN {
                    is_en_passant = match color {
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
                }

                // 5. Add the move to the list. If pawn promotion, add all promotion options instead
                match is_pawn_promotion {
                    true => {
                        for promotion_option in PROMOTION_OPTIONS.iter() {
                            moves.push(Move::new(
                                source_square,
                                target_square,
                                Some(*promotion_option),
                                None,
                                capture,
                                false,
                            ));
                        }
                    }
                    false => moves.push(Move::new(
                        source_square,
                        target_square,
                        None,
                        None,
                        capture,
                        is_en_passant,
                    )),
                }
            }

            // 6. If king, add castling moves (add all now, filter later)
            if piece_type == PieceType::KING {
                let target_squares = [SQUARE::G1, SQUARE::C1, SQUARE::G8, SQUARE::C8];
                let castles = [
                    CASTLE::WhiteKingside,
                    CASTLE::WhiteQueenside,
                    CASTLE::BlackKingside,
                    CASTLE::BlackQueenside,
                ];

                for i in 0..4 {
                    moves.push(Move::new(
                        source_square,
                        target_squares[i],
                        None,
                        Some(castles[i]),
                        None,
                        false,
                    ));
                }
            }
        }

        moves
    }

    pub fn generate_moves_for_color(&self, color: COLOR) -> Vec<Move> {
        vec![
            self.generate_moves_for_piece(PieceType::PAWN.of_color(color)),
            self.generate_moves_for_piece(PieceType::KNIGHT.of_color(color)),
            self.generate_moves_for_piece(PieceType::BISHOP.of_color(color)),
            self.generate_moves_for_piece(PieceType::ROOK.of_color(color)),
            self.generate_moves_for_piece(PieceType::QUEEN.of_color(color)),
            self.generate_moves_for_piece(PieceType::KING.of_color(color)),
        ]
        .into_iter()
        .flatten()
        .filter(|m| self.is_move_valid(*m))
        .collect()
    }
}
