use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct MoveValidator {
    move_: Move,
}

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
            let en_passant_target = board.en_passant_target.expect("en passant target not set");

            // make sure move target and the en passant target are the same
            if target != en_passant_target {
                return false;
            }
        }

        // 6. Invalidate moves where king under check after move
        // we do this by making a copy of the board, making the move, and checking if the king is under attack
        let mut board_copy = board.clone();
        // if no piece at source, print board and panic
        if board_copy.piece_at(source.index()).is_empty() {
            println!("board before move: \n{}", board_copy);
            println!("move: {}", m);
            panic!("no piece at source");
        }

        board_copy.make_move(*m);
        let king_square = SQUARE::from_bits(match source_color {
            COLOR::WHITE => board_copy.white_king.bits(),
            COLOR::BLACK => board_copy.black_king.bits(),
        });
        // println!("board after move: \n{}", board_copy);
        if Self::is_square_under_attack(&board_copy, king_square, source_color) {
            // println!("king under attack");
            // println!("cant do move {:?} -> {:?}", source, target);
            return false;
        }

        // 7. Check for castling
        // this is non-rigorous, since we're relying on data from board generate moves function
        // which is guaranteed to not generate castling moves if castling rights are false
        if source_piece.piece_type() == PieceType::KING {
            // if king currently in check, already returned false

            // for each of the 4 castling moves
            // these are squares strictly between king and rook (source and target)
            let white_kingside = [SQUARE::F1, SQUARE::G1];
            let white_queenside = [SQUARE::D1, SQUARE::C1, SQUARE::B1];
            let black_kingside = [SQUARE::F8, SQUARE::G8];
            let black_queenside = [SQUARE::D8, SQUARE::C8, SQUARE::B8];

            // - check if squares between source and target are empty
            // - check if squares between source and target inclusive are under attack
            match castling {
                Some(CASTLE::WhiteKingside) => {
                    if !white_kingside
                        .iter()
                        .all(|s| board.piece_at(s.index()).is_empty())
                    {
                        return false;
                    }
                    if !white_kingside
                        .iter()
                        .all(|s| !Self::is_square_under_attack(board, *s, source_color))
                    {
                        return false;
                    }
                }
                Some(CASTLE::WhiteQueenside) => {
                    if !white_queenside
                        .iter()
                        .all(|s| board.piece_at(s.index()).is_empty())
                    {
                        return false;
                    }
                    if !white_queenside
                        .iter()
                        .all(|s| !Self::is_square_under_attack(board, *s, source_color))
                    {
                        return false;
                    }
                }
                Some(CASTLE::BlackKingside) => {
                    if !black_kingside
                        .iter()
                        .all(|s| board.piece_at(s.index()).is_empty())
                    {
                        return false;
                    }
                    if !black_kingside
                        .iter()
                        .all(|s| !Self::is_square_under_attack(board, *s, source_color))
                    {
                        return false;
                    }
                }
                Some(CASTLE::BlackQueenside) => {
                    if !black_queenside
                        .iter()
                        .all(|s| board.piece_at(s.index()).is_empty())
                    {
                        return false;
                    }
                    if !black_queenside
                        .iter()
                        .all(|s| !Self::is_square_under_attack(board, *s, source_color))
                    {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    // returns is square under attack for color
    pub fn is_square_under_attack(board: &Board, square: SQUARE, color: COLOR) -> bool {
        // place each of pawn, knight, bishop, rook, queen on king's square and compute attacks
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

        // if any attacks an opposite color pieces of same piecetype, king is under check
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

        // println!("queen attacks:\n{}", queen_attacks);
        // println!(
        //     "{}",
        //     board.pieces_of_color_and_type(color.opposite(), PieceType::QUEEN)
        // );
        // println!("{}", under_queen_attack);
        // println!("above fen: {}", board.to_fen());

        return under_pawn_attack
            || under_knight_attack
            || under_bishop_attack
            || under_rook_attack
            || under_queen_attack;
    }

    pub fn filter_valid_moves(board: &Board, moves: &mut Vec<Move>) {
        moves.retain(|m| Self::is_move_valid(board, m));
    }
}
