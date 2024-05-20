use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct MoveValidator {}

impl MoveValidator {
    // checks if a move is valid given board configuration
    pub fn is_move_valid(board: &Board, m: &Move) -> bool {
        let source = m.source;
        let target = m.target;
        let castling = m.castling;

        // Get details of the piece that is moving
        let source_color = board
            .piece_at(source.index())
            .color()
            .expect("source is empty");
        let target_color = board.piece_at(target.index()).color();

        let source_piece = board.piece_at(source.index());
        let target_piece = board.piece_at(target.index());

        let target_empty = target_piece.piece_type() == PieceType::EMPTY;
        let is_capture = Some(source_color.opposite()) == target_color;

        let rank_diff = (source.rank() - target.rank()).abs();
        let file_diff = (source.file() - target.file()).abs();

        let same_rank = rank_diff == 0;
        let same_file = file_diff == 0;

        let is_pawn = source_piece.piece_type() == PieceType::PAWN;

        let is_pawn_diagonal_move = is_pawn && file_diff == 1 && rank_diff == 1;

        let is_pawn_promotion =
            is_pawn && (target.rank() == RANK::Rank1 || target.rank() == RANK::Rank8);

        // 0. Assert source piece is not empty
        assert_ne!(source_piece, PIECE::Empty);

        // 1. Check for friendly capture
        if Some(source_color) == target_color {
            return false;
        }

        // 2. Check if pawn single push is legal - can't capture forward
        if is_pawn && same_file && rank_diff == 1 && !target_empty {
            return false;
        }

        // 3. Check if pawn double push is legal
        if is_pawn && same_file && rank_diff == 2 {
            let intermediate_square = match source_color {
                COLOR::WHITE => SQUARE::from(source.index() + 8),
                COLOR::BLACK => SQUARE::from(source.index() - 8),
            };
            let intermediate_piece = board.piece_at(intermediate_square.index());
            let intermediate_empty = intermediate_piece.piece_type() == PieceType::EMPTY;

            // if one of the two squares in front of the pawn is occupied, the move is illegal
            if !target_empty || !intermediate_empty {
                return false;
            }
        }

        // 4. Check if the pawn diagonal move is legal (only possible if capture or en passant)
        if is_pawn_diagonal_move && !is_capture && board.en_passant_target.is_none() {
            return false;
        }

        // 5. Check for en passant
        if is_pawn_diagonal_move && !is_capture && board.en_passant_target.is_some() {
            let en_passant_target = board
                .en_passant_target
                .expect("en passant move but en passant target not set");

            // make sure move target and the en passant target are the same
            if target != en_passant_target {
                return false;
            }
        }

        // 6. Invalidate moves where king is under check after move
        // we do this by making a copy of the board, making the move, and checking if the king is under attack
        let mut board_copy = board.clone();
        // if no piece at source, print board and panic
        if board_copy.piece_at(source.index()).is_empty() {
            println!("board before move: \n{}", board_copy);
            println!("move: {}", m);
            panic!("no piece at source");
        }

        // Make move, then if king under attack after move, return false
        board_copy.make_move(*m);
        if Self::in_check(&board_copy, source_color) {
            return false;
        }

        // 7. Check for castling
        // this is non-rigorous, since we're relying on data from board generate moves function
        // which is guaranteed to not generate castling moves if castling rights are false
        if castling.is_some() {
            // Some basic checks to make sure king is moving and that castling rights are set
            assert!(source_piece.piece_type() == PieceType::KING);
            assert!(board.castling_rights.any());

            // add rule for can't castle out of check
            if Self::in_check(board, source_color) {
                return false;
            }

            // for each of the 4 castling moves
            // these are squares strictly between king and rook (source and target)
            // todo turn these into constants
            let wk_side = [SQUARE::F1, SQUARE::G1];
            let wq_side = [SQUARE::D1, SQUARE::C1, SQUARE::B1];
            let bk_side = [SQUARE::F8, SQUARE::G8];
            let bq_side = [SQUARE::D8, SQUARE::C8, SQUARE::B8];

            let wk_side_idx = [SQUARE::F1.index(), SQUARE::G1.index()];
            let wq_side_idx = [SQUARE::D1.index(), SQUARE::C1.index(), SQUARE::B1.index()];
            let bk_side_idx = [SQUARE::F8.index(), SQUARE::G8.index()];
            let bq_side_idx = [SQUARE::D8.index(), SQUARE::C8.index(), SQUARE::B8.index()];

            let wk_side_blocked = wk_side_idx.iter().any(|&i| board.piece_at(i).not_empty());
            let wq_side_blocked = wq_side_idx.iter().any(|&i| board.piece_at(i).not_empty());
            let bk_side_blocked = bk_side_idx.iter().any(|&i| board.piece_at(i).not_empty());
            let bq_side_blocked = bq_side_idx.iter().any(|&i| board.piece_at(i).not_empty());

            let wk_side_attacked = wk_side.iter().any(|s| Self::in_check(board, source_color));
            let wq_side_attacked = wq_side.iter().any(|s| Self::in_check(board, source_color));
            let bk_side_attacked = bk_side.iter().any(|s| Self::in_check(board, source_color));
            let bq_side_attacked = bq_side.iter().any(|s| Self::in_check(board, source_color));

            // - ensure squares between source and target are empty
            // - check if squares between source and target that king moves on are under attack
            match castling {
                Some(CASTLE::WhiteKingside) => {
                    return !wk_side_blocked && !wk_side_attacked;
                }
                Some(CASTLE::WhiteQueenside) => {
                    return !wq_side_blocked && !wq_side_attacked;
                }
                Some(CASTLE::BlackKingside) => {
                    return !bk_side_blocked && !bk_side_attacked;
                }
                Some(CASTLE::BlackQueenside) => {
                    return !bq_side_blocked && !bq_side_attacked;
                }
                _ => {}
            }
        }

        true
    }

    pub fn in_check(board: &Board, color: COLOR) -> bool {
        let king_square = SQUARE::from_bits(match color {
            COLOR::WHITE => board.white_king.bits(),
            COLOR::BLACK => board.black_king.bits(),
        });

        Self::is_square_under_attack(board, king_square, color)
    }

    // returns is square under attack for color
    pub fn is_square_under_attack(board: &Board, square: SQUARE, color: COLOR) -> bool {
        // We do this by placing each of pawn, knight, bishop, rook, queen, and king on the square of interest and computing attacks
        // If any of the target squares is the same piece type as the attacking piece, the king is under attack

        // Start with pawns, but remove single and double push moves
        let pawn_attacks = board.lookup_table.get_pawn_moves(square, color);

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
        let pawn_attacks = pawn_attacks & !(pawn_single_push | pawn_double_push);

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
            & board.pieces_of_color_and_type(color.opposite(), PieceType::PAWN))
        .any();

        let under_knight_attack = (knight_attacks
            & board.pieces_of_color_and_type(color.opposite(), PieceType::KNIGHT))
        .any();

        let under_bishop_attack = (bishop_attacks
            & board.pieces_of_color_and_type(color.opposite(), PieceType::BISHOP))
        .any();

        let under_rook_attack = (rook_attacks
            & board.pieces_of_color_and_type(color.opposite(), PieceType::ROOK))
        .any();

        let under_queen_attack = (queen_attacks
            & board.pieces_of_color_and_type(color.opposite(), PieceType::QUEEN))
        .any();

        let under_king_attack = (king_attacks
            & board.pieces_of_color_and_type(color.opposite(), PieceType::KING))
        .any();

        return under_pawn_attack
            || under_knight_attack
            || under_bishop_attack
            || under_rook_attack
            || under_queen_attack
            || under_king_attack;
    }

    pub fn filter_valid_moves(board: &Board, moves: &mut Vec<Move>) {
        moves.retain(|m| Self::is_move_valid(board, m));
    }
}
