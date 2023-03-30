use crate::*;

#[derive(Debug, Display, Clone, Copy)]
pub enum PIECE {
    WhitePawn = 80,   // P
    WhiteKnight = 78, // N
    WhiteBishop = 66, // B
    WhiteRook = 82,   // R
    WhiteQueen = 81,  // Q
    WhiteKing = 75,   // K

    BlackPawn = 112,   // p
    BlackKnight = 110, // n
    BlackBishop = 98,  // b
    BlackRook = 114,   // r
    BlackQueen = 113,  // q
    BlackKing = 107,   // k

    Empty = 32, // ' '
}

impl PIECE {
    pub fn color(&self) -> COLOR {
        match self {
            PIECE::WhitePawn
            | PIECE::WhiteKnight
            | PIECE::WhiteBishop
            | PIECE::WhiteRook
            | PIECE::WhiteQueen
            | PIECE::WhiteKing => COLOR::WHITE,
            PIECE::BlackPawn
            | PIECE::BlackKnight
            | PIECE::BlackBishop
            | PIECE::BlackRook
            | PIECE::BlackQueen
            | PIECE::BlackKing => COLOR::BLACK,
            PIECE::Empty => panic!("Empty piece has no color"),
        }
    }

    pub fn is_white(&self) -> bool {
        self.color() == COLOR::WHITE
    }

    pub fn is_black(&self) -> bool {
        self.color() == COLOR::BLACK
    }

    pub fn is_color(&self, color: COLOR) -> bool {
        self.not_empty() && self.color() == color
    }

    pub fn is_opposite_color(&self, color: COLOR) -> bool {
        self.not_empty() && self.color() != color
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            PIECE::WhitePawn | PIECE::BlackPawn => true,
            _ => false,
        }
    }

    pub fn is_knight(&self) -> bool {
        match self {
            PIECE::WhiteKnight | PIECE::BlackKnight => true,
            _ => false,
        }
    }

    pub fn is_bishop(&self) -> bool {
        match self {
            PIECE::WhiteBishop | PIECE::BlackBishop => true,
            _ => false,
        }
    }

    pub fn is_rook(&self) -> bool {
        match self {
            PIECE::WhiteRook | PIECE::BlackRook => true,
            _ => false,
        }
    }

    pub fn is_queen(&self) -> bool {
        match self {
            PIECE::WhiteQueen | PIECE::BlackQueen => true,
            _ => false,
        }
    }

    pub fn is_king(&self) -> bool {
        match self {
            PIECE::WhiteKing | PIECE::BlackKing => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PIECE::Empty => true,
            _ => false,
        }
    }

    pub fn not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            PIECE::WhitePawn | PIECE::BlackPawn => PieceType::PAWN,
            PIECE::WhiteKnight | PIECE::BlackKnight => PieceType::KNIGHT,
            PIECE::WhiteBishop | PIECE::BlackBishop => PieceType::BISHOP,
            PIECE::WhiteRook | PIECE::BlackRook => PieceType::ROOK,
            PIECE::WhiteQueen | PIECE::BlackQueen => PieceType::QUEEN,
            PIECE::WhiteKing | PIECE::BlackKing => PieceType::KING,
            PIECE::Empty => PieceType::EMPTY,
        }
    }
}

pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
    EMPTY,
}

impl PieceType {
    pub fn for_color(&self, color: COLOR) -> PIECE {
        match color {
            COLOR::WHITE => match self {
                PieceType::PAWN => PIECE::WhitePawn,
                PieceType::KNIGHT => PIECE::WhiteKnight,
                PieceType::BISHOP => PIECE::WhiteBishop,
                PieceType::ROOK => PIECE::WhiteRook,
                PieceType::QUEEN => PIECE::WhiteQueen,
                PieceType::KING => PIECE::WhiteKing,
                PieceType::EMPTY => PIECE::Empty,
            },
            COLOR::BLACK => match self {
                PieceType::PAWN => PIECE::BlackPawn,
                PieceType::KNIGHT => PIECE::BlackKnight,
                PieceType::BISHOP => PIECE::BlackBishop,
                PieceType::ROOK => PIECE::BlackRook,
                PieceType::QUEEN => PIECE::BlackQueen,
                PieceType::KING => PIECE::BlackKing,
                PieceType::EMPTY => PIECE::Empty,
            },
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum COLOR {
    BLACK = 0,
    WHITE = 1,
}

// ranks and files are defined in the following way so that rank * 8 + file = square
#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq)]
pub enum RANK {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

impl RANK {
    pub fn bits(&self) -> u64 {
        match self {
            RANK::Rank1 => 0xFF,     // 0b11111111
            RANK::Rank2 => 0xFF00,   // 0b1111111100000000
            RANK::Rank3 => 0xFF0000, // ...
            RANK::Rank4 => 0xFF000000,
            RANK::Rank5 => 0xFF00000000,
            RANK::Rank6 => 0xFF0000000000,
            RANK::Rank7 => 0xFF000000000000,
            RANK::Rank8 => 0xFF00000000000000,
        }
    }
}

#[derive(Debug, Display, EnumIter, PartialEq)]
pub enum FILE {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}

impl FILE {
    pub fn bits(&self) -> u64 {
        match self {
            FILE::FileA => 0x0101010101010101,
            FILE::FileB => 0x0202020202020202,
            FILE::FileC => 0x0404040404040404,
            FILE::FileD => 0x0808080808080808,
            FILE::FileE => 0x1010101010101010,
            FILE::FileF => 0x2020202020202020,
            FILE::FileG => 0x4040404040404040,
            FILE::FileH => 0x8080808080808080,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
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

#[derive(Debug, Clone, Copy)]
pub enum CASTLE {
    WhiteKingside = 0,
    WhiteQueenside = 1,
    BlackKingside = 2,
    BlackQueenside = 3,
}

#[derive(Debug, Display, Clone, Copy, EnumIter)]
pub enum DIRECTION {
    NORTH = 8,
    SOUTH = -8,
    EAST = 1,
    WEST = -1,
}

impl DIRECTION {
    pub fn bits(&self) -> u64 {
        (*self as i8).abs() as u64
    }
}

// Square enum is defined in the following way because bitshifting becomes more convenient
// ex. to get the u64 rep of a1 we can do (1 << (A1 as usize))
// this helps with things like generating moves
// ex. to get north we do (1 << (A1 as usize)) << 8

#[derive(Debug, Display, Clone, Copy, EnumIter)]
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
