use crate::enums::*;

#[derive(Debug, Clone, Copy)]
pub struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl CastlingRights {
    pub fn from_fen(fen: &str) -> Self {
        let mut castling_rights = CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        };
        for c in fen.chars() {
            match c {
                'K' => castling_rights.white_kingside = true,
                'Q' => castling_rights.white_queenside = true,
                'k' => castling_rights.black_kingside = true,
                'q' => castling_rights.black_queenside = true,
                _ => (),
            }
        }
        castling_rights
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        if self.white_kingside {
            fen.push('K');
        }
        if self.white_queenside {
            fen.push('Q');
        }
        if self.black_kingside {
            fen.push('k');
        }
        if self.black_queenside {
            fen.push('q');
        }
        if fen.is_empty() {
            fen.push('-');
        }
        fen
    }

    pub fn any(&self) -> bool {
        self.white_kingside || self.white_queenside || self.black_kingside || self.black_queenside
    }

    pub fn any_white(&self) -> bool {
        self.white_kingside || self.white_queenside
    }

    pub fn any_black(&self) -> bool {
        self.black_kingside || self.black_queenside
    }

    pub fn get(&self, castle: CASTLE) -> bool {
        match castle {
            CASTLE::WhiteKingside => self.white_kingside,
            CASTLE::WhiteQueenside => self.white_queenside,
            CASTLE::BlackKingside => self.black_kingside,
            CASTLE::BlackQueenside => self.black_queenside,
        }
    }

    pub fn set_index(&mut self, index: usize, value: bool) {
        let castle = match index {
            0 => CASTLE::WhiteKingside,
            1 => CASTLE::WhiteQueenside,
            2 => CASTLE::BlackKingside,
            3 => CASTLE::BlackQueenside,
            _ => panic!("Invalid index"),
        };

        self.set(castle, value);
    }

    pub fn set(&mut self, castle: CASTLE, value: bool) {
        match castle {
            CASTLE::WhiteKingside => self.white_kingside = value,
            CASTLE::WhiteQueenside => self.white_queenside = value,
            CASTLE::BlackKingside => self.black_kingside = value,
            CASTLE::BlackQueenside => self.black_queenside = value,
        }
    }

    pub fn remove_color(&mut self, color: COLOR) {
        match color {
            COLOR::WHITE => {
                self.set(CASTLE::WhiteQueenside, false);
                self.set(CASTLE::WhiteKingside, false);
            }
            COLOR::BLACK => {
                self.set(CASTLE::BlackKingside, false);
                self.set(CASTLE::BlackQueenside, false);
            }
        }
    }

    pub fn give_color(&mut self, color: COLOR) {
        match color {
            COLOR::WHITE => {
                self.set(CASTLE::WhiteKingside, true);
                self.set(CASTLE::WhiteQueenside, true);
            }
            COLOR::BLACK => {
                self.set(CASTLE::BlackKingside, true);
                self.set(CASTLE::BlackQueenside, true);
            }
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}
