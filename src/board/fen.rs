use crate::board::Board;
use crate::enums::*;
use crate::lookup_table::LookupTable;
use strum::IntoEnumIterator;

impl<'a> Board<'a> {
    pub fn from_fen(fen: &str, lookup_table: &'a LookupTable) -> Board<'a> {
        let mut board = Board::new(lookup_table);

        // split the board configuration from metadata
        let fen = fen.trim().split(" ").into_iter().collect::<Vec<&str>>();
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
}
