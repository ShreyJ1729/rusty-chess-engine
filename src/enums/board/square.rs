use crate::enums::*;
use strum_macros::Display;
use strum_macros::EnumIter;

// Square enum is defined in the following way because bitshifting becomes more convenient
// ex. to get the u64 rep of a1 we can do (1 << (A1 as usize))
// this helps with things like generating moves
// ex. to get north we do (1 << (A1 as usize)) << 8

#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq)]
pub enum SQUARE {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

impl SQUARE {
    pub fn bits(&self) -> u64 {
        1 << self.index()
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn rank(&self) -> RANK {
        match self.index() {
            0..=7 => RANK::Rank1,
            8..=15 => RANK::Rank2,
            16..=23 => RANK::Rank3,
            24..=31 => RANK::Rank4,
            32..=39 => RANK::Rank5,
            40..=47 => RANK::Rank6,
            48..=55 => RANK::Rank7,
            56..=63 => RANK::Rank8,
            _ => panic!("Invalid square"),
        }
    }

    pub fn file(&self) -> FILE {
        match self.index() {
            0 | 8 | 16 | 24 | 32 | 40 | 48 | 56 => FILE::FileA,
            1 | 9 | 17 | 25 | 33 | 41 | 49 | 57 => FILE::FileB,
            2 | 10 | 18 | 26 | 34 | 42 | 50 | 58 => FILE::FileC,
            3 | 11 | 19 | 27 | 35 | 43 | 51 | 59 => FILE::FileD,
            4 | 12 | 20 | 28 | 36 | 44 | 52 | 60 => FILE::FileE,
            5 | 13 | 21 | 29 | 37 | 45 | 53 | 61 => FILE::FileF,
            6 | 14 | 22 | 30 | 38 | 46 | 54 | 62 => FILE::FileG,
            7 | 15 | 23 | 31 | 39 | 47 | 55 | 63 => FILE::FileH,
            _ => panic!("Invalid square"),
        }
    }

    pub fn diagonal(&self) -> DIAGONAL {
        match self.index() {
            56 => DIAGONAL::A8A8,
            48 | 57 => DIAGONAL::A7B8,
            40 | 49 | 58 => DIAGONAL::A6C8,
            32 | 41 | 50 | 59 => DIAGONAL::A5D8,
            24 | 33 | 42 | 51 | 60 => DIAGONAL::A4E8,
            16 | 25 | 34 | 43 | 52 | 61 => DIAGONAL::A3F8,
            8 | 17 | 26 | 35 | 44 | 53 | 62 => DIAGONAL::A2G8,

            0 | 9 | 18 | 27 | 36 | 45 | 54 | 63 => DIAGONAL::A1H8,

            1 | 10 | 19 | 28 | 37 | 46 | 55 => DIAGONAL::B1H7,
            2 | 11 | 20 | 29 | 38 | 47 => DIAGONAL::C1H6,
            3 | 12 | 21 | 30 | 39 => DIAGONAL::D1H5,
            4 | 13 | 22 | 31 => DIAGONAL::E1H4,
            5 | 14 | 23 => DIAGONAL::F1H3,
            6 | 15 => DIAGONAL::G1H2,
            7 => DIAGONAL::H1H1,
            _ => panic!("Invalid square"),
        }
    }

    pub fn antidiagonal(&self) -> ANTIDIAGONAL {
        match self.index() {
            0 => ANTIDIAGONAL::A1A1,
            1 | 8 => ANTIDIAGONAL::A2B1,
            2 | 9 | 16 => ANTIDIAGONAL::A3C1,
            3 | 10 | 17 | 24 => ANTIDIAGONAL::A4D1,
            4 | 11 | 18 | 25 | 32 => ANTIDIAGONAL::A5E1,
            5 | 12 | 19 | 26 | 33 | 40 => ANTIDIAGONAL::A6F1,
            6 | 13 | 20 | 27 | 34 | 41 | 48 => ANTIDIAGONAL::A7G1,

            7 | 14 | 21 | 28 | 35 | 42 | 49 | 56 => ANTIDIAGONAL::A8H1,

            15 | 22 | 29 | 36 | 43 | 50 | 57 => ANTIDIAGONAL::B8H2,
            23 | 30 | 37 | 44 | 51 | 58 => ANTIDIAGONAL::C8H3,
            31 | 38 | 45 | 52 | 59 => ANTIDIAGONAL::D8H4,
            39 | 46 | 53 | 60 => ANTIDIAGONAL::E8H5,
            47 | 54 | 61 => ANTIDIAGONAL::F8H6,
            55 | 62 => ANTIDIAGONAL::G8H7,
            63 => ANTIDIAGONAL::H8H8,
            _ => panic!("Invalid square"),
        }
    }

    pub fn is_pawn_start(&self, color: COLOR) -> bool {
        match color {
            COLOR::WHITE => self.rank() == RANK::Rank2,
            COLOR::BLACK => self.rank() == RANK::Rank7,
        }
    }

    pub fn is_pawn_promote(&self, color: COLOR) -> bool {
        match color {
            COLOR::WHITE => self.rank() == RANK::Rank8,
            COLOR::BLACK => self.rank() == RANK::Rank1,
        }
    }

    // can only be used on bitboards with a single bit set
    pub fn from_bits(bits: u64) -> SQUARE {
        assert!(bits == 0 || bits.count_ones() == 1);
        SQUARE::from(bits.trailing_zeros() as usize)
    }

    pub fn from_string(s: &str) -> Option<SQUARE> {
        if s.len() != 2 {
            return None;
        }

        let file = match s.chars().nth(0).unwrap() {
            'a' => FILE::FileA,
            'b' => FILE::FileB,
            'c' => FILE::FileC,
            'd' => FILE::FileD,
            'e' => FILE::FileE,
            'f' => FILE::FileF,
            'g' => FILE::FileG,
            'h' => FILE::FileH,
            _ => return None,
        };

        let rank = match s.chars().nth(1).unwrap() {
            '1' => RANK::Rank1,
            '2' => RANK::Rank2,
            '3' => RANK::Rank3,
            '4' => RANK::Rank4,
            '5' => RANK::Rank5,
            '6' => RANK::Rank6,
            '7' => RANK::Rank7,
            '8' => RANK::Rank8,
            _ => return None,
        };

        Some(SQUARE::from(file.index() * 8 + rank.index()))
    }

    pub fn to_fen(&self) -> String {
        format!("{}{}", self.file().to_fen(), self.rank().to_fen())
    }

    pub fn north(&self) -> Option<SQUARE> {
        match self.rank() {
            RANK::Rank8 => None,
            _ => Some(SQUARE::from(self.index() + 8)),
        }
    }

    pub fn south(&self) -> Option<SQUARE> {
        match self.rank() {
            RANK::Rank1 => None,
            _ => Some(SQUARE::from(self.index() - 8)),
        }
    }

    pub fn east(&self) -> Option<SQUARE> {
        match self.file() {
            FILE::FileH => None,
            _ => Some(SQUARE::from(self.index() + 1)),
        }
    }

    pub fn west(&self) -> Option<SQUARE> {
        match self.file() {
            FILE::FileA => None,
            _ => Some(SQUARE::from(self.index() - 1)),
        }
    }
}

impl From<usize> for SQUARE {
    fn from(item: usize) -> Self {
        match item {
            0 => SQUARE::A1,
            1 => SQUARE::B1,
            2 => SQUARE::C1,
            3 => SQUARE::D1,
            4 => SQUARE::E1,
            5 => SQUARE::F1,
            6 => SQUARE::G1,
            7 => SQUARE::H1,
            8 => SQUARE::A2,
            9 => SQUARE::B2,
            10 => SQUARE::C2,
            11 => SQUARE::D2,
            12 => SQUARE::E2,
            13 => SQUARE::F2,
            14 => SQUARE::G2,
            15 => SQUARE::H2,
            16 => SQUARE::A3,
            17 => SQUARE::B3,
            18 => SQUARE::C3,
            19 => SQUARE::D3,
            20 => SQUARE::E3,
            21 => SQUARE::F3,
            22 => SQUARE::G3,
            23 => SQUARE::H3,
            24 => SQUARE::A4,
            25 => SQUARE::B4,
            26 => SQUARE::C4,
            27 => SQUARE::D4,
            28 => SQUARE::E4,
            29 => SQUARE::F4,
            30 => SQUARE::G4,
            31 => SQUARE::H4,
            32 => SQUARE::A5,
            33 => SQUARE::B5,
            34 => SQUARE::C5,
            35 => SQUARE::D5,
            36 => SQUARE::E5,
            37 => SQUARE::F5,
            38 => SQUARE::G5,
            39 => SQUARE::H5,
            40 => SQUARE::A6,
            41 => SQUARE::B6,
            42 => SQUARE::C6,
            43 => SQUARE::D6,
            44 => SQUARE::E6,
            45 => SQUARE::F6,
            46 => SQUARE::G6,
            47 => SQUARE::H6,
            48 => SQUARE::A7,
            49 => SQUARE::B7,
            50 => SQUARE::C7,
            51 => SQUARE::D7,
            52 => SQUARE::E7,
            53 => SQUARE::F7,
            54 => SQUARE::G7,
            55 => SQUARE::H7,
            56 => SQUARE::A8,
            57 => SQUARE::B8,
            58 => SQUARE::C8,
            59 => SQUARE::D8,
            60 => SQUARE::E8,
            61 => SQUARE::F8,
            62 => SQUARE::G8,
            63 => SQUARE::H8,
            _ => panic!("Invalid square"),
        }
    }
}
