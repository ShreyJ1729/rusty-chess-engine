use crate::board::Board;
use crate::enums::ENDGAME;

impl<'a> Board<'a> {
    pub fn insufficient_material(&self) -> bool {
        let white_pieces = self.white_occupancy().count();
        let black_pieces = self.black_occupancy().count();

        // If both sides only have one of the following its insufficient material
        // - lone king
        // - king and bishop
        // - king and knight

        if white_pieces <= 2 && black_pieces <= 2 {
            let w_has_lone_k = white_pieces == 1;
            let b_has_lone_k = black_pieces == 1;
            let w_has_b = self.white_bishops.not_empty();
            let w_has_n = self.white_knights.not_empty();
            let b_has_b = self.black_bishops.not_empty();
            let b_has_n = self.black_knights.not_empty();

            let b_has_one_of_three = b_has_b | b_has_n | b_has_lone_k;
            let w_has_one_of_three = w_has_b | w_has_n | w_has_lone_k;

            return w_has_one_of_three && b_has_one_of_three;
        }

        false
    }

    pub fn threefold_repetition(&self) -> bool {
        // todo implement
        false
    }

    pub fn endgame(&self) -> Option<ENDGAME> {
        let moves = self.generate_moves_for_color(self.to_move);
        let in_check = self.in_check(self.to_move);
        let no_moves = moves.is_empty();

        // 1. Checkmate
        if no_moves && in_check {
            return Some(ENDGAME::Checkmate);
        }

        // 2. Stalemate
        if no_moves && !in_check {
            return Some(ENDGAME::Stalemate);
        }

        // 3. Insufficient material
        if self.insufficient_material() {
            return Some(ENDGAME::InsufficientMaterial);
        }

        // 4. Fifty-move rule
        if self.halfmove_clock >= 100 {
            return Some(ENDGAME::FiftyMoveRule);
        }

        // 5. Threefold repetition
        if self.threefold_repetition() {
            return Some(ENDGAME::ThreefoldRepetition);
        }

        None
    }
}
