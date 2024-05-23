use crate::{bitboard::*, board::*, enums::*, helpers::*, r#move::*};

#[derive(Debug, Clone, Copy)]
pub struct MoveValidator {}

impl MoveValidator {
    // checks if a move is valid given board configuration
    pub fn is_move_valid(board: &Board, m: Move) -> bool {
        let source = m.source;
        let target = m.target;
        let castling = m.castling;

        // Get details of the piece that is moving
        let source_piece = board.piece_at(source);
        let target_piece = board.piece_at(target);

        let source_color = source_piece.color().expect("source piece is empty");
        let target_color = target_piece.color();

        let target_empty = target_piece == PIECE::Empty;
        let is_capture = Some(source_color.opposite()) == target_color;
        let friendly_capture = Some(source_color) == target_color;

        let rank_diff = (source.rank() - target.rank()).abs();
        let file_diff = (source.file() - target.file()).abs();

        let rank_unchanged = rank_diff == 0;
        let file_unchanged = file_diff == 0;

        let pawn_move = source_piece.piece_type() == PieceType::PAWN;
        let pawn_single_push = pawn_move && rank_diff == 1 && file_unchanged;
        let pawn_double_push = pawn_move && rank_diff == 2 && file_unchanged;
        let pawn_diagonal_move = pawn_move && file_diff == 1 && rank_diff == 1;

        let pawn_promoted =
            pawn_move && (target.rank() == RANK::Rank1 || target.rank() == RANK::Rank8);

        // 0. Assert source piece is not empty
        assert_ne!(source_piece, PIECE::Empty);

        // 1. Check for friendly capture or king capture
        let king_capture = target_piece.piece_type() == PieceType::KING;
        if friendly_capture || king_capture {
            return false;
        }

        // 2. Check if pawn single push is legal - can't capture forward
        if pawn_single_push && is_capture {
            return false;
        }

        // 3. Check if pawn double push is legal
        if pawn_double_push {
            let intermediate_square = match source_color {
                COLOR::WHITE => source.north().unwrap(),
                COLOR::BLACK => source.south().unwrap(),
            };
            let intermediate_piece = board.piece_at(intermediate_square);
            let intermediate_empty = intermediate_piece == PIECE::Empty;

            // if target or intermediate square is occupied, return false
            if !target_empty || !intermediate_empty {
                return false;
            }
        }

        // 4. Check if the pawn diagonal move is legal (only possible if capture or en passant)
        if pawn_diagonal_move && !is_capture && board.en_passant_target.is_none() {
            return false;
        }

        // 5. Check for en passant
        if pawn_diagonal_move && !is_capture && board.en_passant_target.is_some() {
            let en_passant_target = board
                .en_passant_target
                .expect("en passant move but en passant target not set");

            // make sure move target and the en passant target are the same
            if target != en_passant_target {
                return false;
            }
        }

        // 6. Invalidate moves where king is under check after move
        let mut board_copy = board.clone();
        board_copy.make_move(m);

        if Self::in_check(&board_copy, source_color) {
            return false;
        }

        // 7. Check for castling
        if castling.is_some() {
            assert!(source_piece.piece_type() == PieceType::KING);

            // no castling if king is in check
            if Self::in_check(board, source_color) {
                return false;
            }

            let wkc_blocked = (WKC_BITS & board.occupancy().bits()) != 0;
            let wqc_blocked = (WQC_BITS & board.occupancy().bits()) != 0;
            let bkc_blocked = (BKC_BITS & board.occupancy().bits()) != 0;
            let bqc_blocked = (BQC_BITS & board.occupancy().bits()) != 0;

            let wkc_attacked = WKC_SQUARES
                .iter()
                .any(|s| Self::square_under_attack(board, *s, source_color));
            let wqc_attacked = WQC_SQUARES
                .iter()
                .take(2)
                .any(|s| Self::square_under_attack(board, *s, source_color));
            let bkc_attacked = BKC_SQUARES
                .iter()
                .any(|s| Self::square_under_attack(board, *s, source_color));
            let bqc_attacked = BQC_SQUARES
                .iter()
                .take(2)
                .any(|s| Self::square_under_attack(board, *s, source_color));

            match castling {
                Some(CASTLE::WhiteKingside) => {
                    return !wkc_blocked && !wkc_attacked && board.castling_rights.white_kingside;
                }
                Some(CASTLE::WhiteQueenside) => {
                    return !wqc_blocked && !wqc_attacked && board.castling_rights.white_queenside;
                }
                Some(CASTLE::BlackKingside) => {
                    return !bkc_blocked && !bkc_attacked && board.castling_rights.black_kingside;
                }
                Some(CASTLE::BlackQueenside) => {
                    return !bqc_blocked && !bqc_attacked && board.castling_rights.black_queenside;
                }
                _ => {}
            }
        }

        true
    }

    pub fn either_color_in_check(board: &Board) -> bool {
        Self::in_check(board, COLOR::WHITE) || Self::in_check(board, COLOR::BLACK)
    }

    pub fn in_check(board: &Board, color: COLOR) -> bool {
        let king_square = SQUARE::from_bits(match color {
            COLOR::WHITE => board.white_king.bits(),
            COLOR::BLACK => board.black_king.bits(),
        });

        Self::square_under_attack(board, king_square, color)
    }

    // returns is square under attack by the opposite color
    pub fn square_under_attack(board: &Board, square: SQUARE, color: COLOR) -> bool {
        // We do this by placing each of pawn, knight, bishop, rook, queen, and king on the square of interest and computing attacks
        // If any of the target squares is the same piece type as the attacking piece, the king is under attack

        // Start with pawns, but remove single and double push moves
        let pawn_moves = board.lookup_table.get_pawn_moves(square, color);

        // for pawns, remove single and double push moves from attacks
        let pawn_single_push = Bitboard::new(match color {
            COLOR::WHITE => north(square.bits()).unwrap_or(0),
            COLOR::BLACK => south(square.bits()).unwrap_or(0),
        });
        let pawn_double_push = Bitboard::new(match color {
            COLOR::WHITE => north_north(square.bits()).unwrap_or(0),
            COLOR::BLACK => south_south(square.bits()).unwrap_or(0),
        });

        // Now we can compute only diagonal pawn moves
        let pawn_attacks = pawn_moves & !(pawn_single_push | pawn_double_push);

        // rest are straightforward
        let knight_attacks = board.lookup_table.get_knight_moves(square, color);
        let bishop_attacks =
            board
                .lookup_table
                .get_bishop_moves(square, color, board.occupancy().bits());
        let rook_attacks =
            board
                .lookup_table
                .get_rook_moves(square, color, board.occupancy().bits());
        let queen_attacks =
            board
                .lookup_table
                .get_queen_moves(square, color, board.occupancy().bits());
        let king_attacks = board.lookup_table.get_king_moves(square, color);

        // if any attacks are on opposite color pieces of same piecetype, the square is under attack
        let under_pawn_attack = (pawn_attacks
            & board.occupancy_of_piece(PieceType::PAWN.for_color(color.opposite())))
        .any();

        let under_knight_attack = (knight_attacks
            & board.occupancy_of_piece(PieceType::KNIGHT.for_color(color.opposite())))
        .any();

        let under_bishop_attack = (bishop_attacks
            & board.occupancy_of_piece(PieceType::BISHOP.for_color(color.opposite())))
        .any();

        let under_rook_attack = (rook_attacks
            & board.occupancy_of_piece(PieceType::ROOK.for_color(color.opposite())))
        .any();

        let under_queen_attack = (queen_attacks
            & board.occupancy_of_piece(PieceType::QUEEN.for_color(color.opposite())))
        .any();

        let under_king_attack = (king_attacks
            & board.occupancy_of_piece(PieceType::KING.for_color(color.opposite())))
        .any();

        return under_pawn_attack
            || under_knight_attack
            || under_bishop_attack
            || under_rook_attack
            || under_queen_attack
            || under_king_attack;
    }

    pub fn filter_valid_moves(board: &Board, moves: &mut Vec<Move>) {
        moves.retain(|m| Self::is_move_valid(board, *m));
    }
}
