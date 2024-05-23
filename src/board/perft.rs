use crate::board::Board;
use crate::enums::ENDGAME;
use std::io::Write;

impl<'a> Board<'a> {
    pub fn perft(
        &mut self,
        depth: u8,
        max_depth: u8,
        node_only: bool,
    ) -> (u64, u64, u64, u64, u64, u64, u64) {
        let mut nodes = 0;
        let mut captures = 0;
        let mut en_passants = 0;
        let mut castles = 0;
        let mut promotions = 0;
        let mut checks = 0;
        let mut checkmates = 0;

        if depth == 0 {
            return (1, 0, 0, 0, 0, 0, 0);
        }

        let moves = self.generate_moves_for_color(self.to_move);

        if depth == max_depth {
            print!("(0/{}) |>{}|\r", moves.len(), " ".repeat(moves.len()));
            std::io::stdout().flush().unwrap();
        }

        for (i, m) in moves.iter().enumerate() {
            let mut board = self.clone();
            board.make_move(*m);

            if depth == 1 {
                nodes += 1;
                captures += (m.capture.is_some() | m.en_passant) as u64;
                en_passants += m.en_passant as u64;
                castles += m.castling.is_some() as u64;
                promotions += m.promotion.is_some() as u64;
                checks += board.in_check(board.to_move) as u64;
                checkmates += (board.endgame() == Some(ENDGAME::Checkmate)) as u64;
            } else {
                let (n, c, en, ca, pro, ch, cm) = board.perft(depth - 1, max_depth, node_only);
                nodes += n;
                captures += c;
                en_passants += en;
                castles += ca;
                promotions += pro;
                checks += ch;
                checkmates += cm;
            }

            if depth == max_depth {
                let progress = "=".repeat((i + 1) as usize);
                let empty = " ".repeat(moves.len() - i - 1);
                print!("({}/{}) |{}>{}|\r", i + 1, moves.len(), progress, empty);
                std::io::stdout().flush().unwrap();
            }
        }
        (
            nodes,
            captures,
            en_passants,
            castles,
            promotions,
            checks,
            checkmates,
        )
    }
}
