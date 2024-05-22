use strum_macros::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum CASTLE {
    WhiteKingside = 0,
    WhiteQueenside = 1,
    BlackKingside = 2,
    BlackQueenside = 3,
}
