use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct MoveValidator {}

impl MoveValidator {
    pub fn new() -> Self {
        Self {}
    }

    // checks if a move is valid given board configuration
    pub fn is_move_valid(&self, board: &Board, m: &Move) -> bool {
        let source = m.source;
        let target = m.target;

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
        if is_pawn_diagonal_move && !is_capture && !board.en_passant_available {
            return false;
        }

        // 5. Check for en passant
        if is_pawn_diagonal_move && !is_capture && board.en_passant_available {
            let en_passant_target = board.en_passant_target.expect("en passant target not set");

            // make sure move target and the en passant target are the same
            if target != en_passant_target {
                return false;
            }
        }

        // 6. Check for castling
        if source_piece.piece_type() == PieceType::KING {
            // TODO:
            // if king in check, return false
            // for each of the 4 castling moves
            // - check if source/target squares are correct
            // - check board state for castling rights
            // - check if squares between source and target are empty
            // - check if squares between source and target are under attack
            // - - compute all opponent moves + check if those squares are a target
        }

        // 7. Check if king under check after move
        if source_piece.piece_type() == PieceType::KING {
            // place each of pawn, knight, bishop, rook, queen on king's square and compute attacks
            let pawn_attacks = board.lookup_table.get_pawn_moves(target, source_color);
            let knight_attacks = board.lookup_table.get_knight_moves(target, source_color);
            let bishop_attacks =
                board
                    .lookup_table
                    .get_bishop_moves(target, source_color, board.occupancy().bits());
            let rook_attacks =
                board
                    .lookup_table
                    .get_rook_moves(target, source_color, board.occupancy().bits());
            let queen_attacks =
                board
                    .lookup_table
                    .get_queen_moves(target, source_color, board.occupancy().bits());

            // if any attacks an opposite color pieces of same piecetype, king is under check
            let under_pawn_attack = (pawn_attacks
                & board.pieces_of_color_and_type(source_color.opposite(), PieceType::PAWN))
            .any();

            let under_knight_attack = (knight_attacks
                & board.pieces_of_color_and_type(source_color.opposite(), PieceType::KNIGHT))
            .any();

            let under_bishop_attack = (bishop_attacks
                & board.pieces_of_color_and_type(source_color.opposite(), PieceType::BISHOP))
            .any();

            let under_rook_attack = (rook_attacks
                & board.pieces_of_color_and_type(source_color.opposite(), PieceType::ROOK))
            .any();

            let under_queen_attack = (queen_attacks
                & board.pieces_of_color_and_type(source_color.opposite(), PieceType::QUEEN))
            .any();

            if under_pawn_attack
                || under_knight_attack
                || under_bishop_attack
                || under_rook_attack
                || under_queen_attack
            {
                return false;
            }
        }

        true
    }

    pub fn filter_valid_moves(&self, board: &Board, moves: &mut Vec<Move>) {
        moves.retain(|m| self.is_move_valid(board, m));
    }
}
