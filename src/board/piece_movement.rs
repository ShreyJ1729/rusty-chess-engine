use crate::board::Board;
use crate::enums::*;
use crate::helpers::*;
use crate::r#move::Move;

impl<'a> Board<'a> {
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
                        self.add_piece(target_index, new_piece.of_color(source_color));
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
    }
}
