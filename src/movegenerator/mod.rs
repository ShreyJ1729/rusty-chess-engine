use crate::*;

use super::enums::COLOR;

pub struct MoveGenerator<'a> {
    board: &'a Board,
}

impl MoveGenerator<'_> {
    pub fn new(board: &Board) -> MoveGenerator {
        MoveGenerator { board }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        SQUARE::iter()
            .filter(|square| {
                let piece = self.board.piece_at(square.bits());
                piece.not_empty()
            })
            .map(|square| self.generate_moves_for_square(square))
            .flatten()
            .collect()
    }

    pub fn generate_moves_for_square(&self, square: SQUARE) -> Vec<Move> {
        let piece = self.board.piece_at(square.bits());
        let color = piece.color();

        match piece.not_empty() {
            true => {
                let moves = match piece.piece_type() {
                    PieceType::PAWN => self.generate_pawn_moves(square, color),
                    PieceType::KNIGHT => self.generate_knight_moves(square, color),
                    PieceType::BISHOP => self.generate_bishop_moves(square, color),
                    PieceType::ROOK => self.generate_rook_moves(square, color),
                    PieceType::QUEEN => self.generate_queen_moves(square, color),
                    PieceType::KING => self.generate_king_moves(square, color),
                    PieceType::EMPTY => vec![],
                };

                if moves.len() > 0 {
                    println!("{} moves for {} at {}", moves.len(), piece, square);
                }

                moves
            }
            false => vec![],
        }
    }

    pub fn bits_to_move(&self, source: u64, target: u64, piece: PIECE, color: COLOR) -> Move {
        let source_square = u64_to_square(source);
        let target_square = u64_to_square(target);
        match piece {
            PIECE::WhitePawn => {
                if target_square.is_pawn_promote(color) {
                    Move::new(source, target, Some(PIECE::WhiteQueen))
                } else {
                    Move::new(source, target, None)
                }
            }
            PIECE::BlackPawn => {
                if target_square.is_pawn_promote(color) {
                    Move::new(source, target, Some(PIECE::BlackQueen))
                } else {
                    Move::new(source, target, None)
                }
            }
            PIECE::WhiteKnight => Move::new(source, target, None),
            PIECE::BlackKnight => Move::new(source, target, None),
            PIECE::WhiteBishop => Move::new(source, target, None),
            PIECE::BlackBishop => Move::new(source, target, None),
            PIECE::WhiteRook => Move::new(source, target, None),
            PIECE::BlackRook => Move::new(source, target, None),
            PIECE::WhiteQueen => Move::new(source, target, None),
            PIECE::BlackQueen => Move::new(source, target, None),
            PIECE::WhiteKing => Move::new(source, target, None),
            PIECE::BlackKing => Move::new(source, target, None),
            PIECE::Empty => panic!("Empty piece"),
        }
    }

    // todo add castling using self.board.castling_rights
    pub fn generate_king_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        let source = square.bits();

        let targets = vec![
            north(source),
            north_east(source),
            east(source),
            south_east(source),
            south(source),
            south_west(source),
            west(source),
            north_west(source),
        ];

        let moves = targets
            .into_iter()
            .filter_map(|target| target)
            .map(|target| self.bits_to_move(source, target, PIECE::WhiteKing, color))
            .collect();

        moves
    }

    pub fn generate_knight_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        let source = square.bits();

        let targets = vec![
            north_north_east(source),
            east_east_north(source),
            east_east_south(source),
            south_south_east(source),
            south_south_west(source),
            west_west_south(source),
            west_west_north(source),
            north_north_west(source),
        ];

        let moves = targets
            .into_iter()
            .filter_map(|target| target)
            .map(|target| self.bits_to_move(source, target, PIECE::WhiteKnight, color))
            .collect();

        moves
    }

    pub fn generate_rook_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        let source = square.bits();

        let mut targets = vec![];

        // keep adding direction to targets until direction returns None (hit edge)
        let mut north_target = north(source);
        let mut south_target = south(source);
        let mut east_target = east(source);
        let mut west_target = west(source);

        while let Some(target) = north_target {
            targets.push(target);
            north_target = north(target);
        }

        while let Some(target) = south_target {
            targets.push(target);
            south_target = south(target);
        }

        while let Some(target) = east_target {
            targets.push(target);
            east_target = east(target);
        }

        while let Some(target) = west_target {
            targets.push(target);
            west_target = west(target);
        }

        let moves = targets
            .into_iter()
            .map(|target| self.bits_to_move(source, target, PIECE::WhiteRook, color))
            .collect();

        moves
    }

    pub fn generate_bishop_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        vec![]
    }

    pub fn generate_queen_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        let mut moves = self.generate_rook_moves(square, color);
        moves.append(&mut self.generate_bishop_moves(square, color));
        moves
    }

    // todo add en passant
    pub fn generate_pawn_moves(&self, square: SQUARE, color: COLOR) -> Vec<Move> {
        let source = square.bits();

        let forward_move = match color {
            COLOR::WHITE => match north(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
            COLOR::BLACK => match south(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
        };

        let double_move = match square.is_pawn_start(color) {
            true => match color {
                COLOR::WHITE => match north_north(source) {
                    Some(target) => Some(Move::new(source, target, None)),
                    None => None,
                },
                COLOR::BLACK => match south_south(source) {
                    Some(target) => Some(Move::new(source, target, None)),
                    None => None,
                },
            },
            false => None,
        };
        let left_capture = match color {
            COLOR::WHITE => match north_west(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
            COLOR::BLACK => match south_west(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
        };
        let right_capture = match color {
            COLOR::WHITE => match north_east(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
            COLOR::BLACK => match south_east(source) {
                Some(target) => Some(Move::new(source, target, None)),
                None => None,
            },
        };

        if square.is_pawn_promote(color) {
            let promotion_piece = Some(PieceType::QUEEN.for_color(color));
            forward_move.and_then(|mut move_| Some(move_.promotion = promotion_piece));
            double_move.and_then(|mut move_| Some(move_.promotion = promotion_piece));
            left_capture.and_then(|mut move_| Some(move_.promotion = promotion_piece));
        }

        let mut moves = Vec::new();

        forward_move.and_then(|move_| Some(moves.push(move_)));
        double_move.and_then(|move_| Some(moves.push(move_)));
        left_capture.and_then(|move_| Some(moves.push(move_)));
        right_capture.and_then(|move_| Some(moves.push(move_)));

        moves
    }
}
