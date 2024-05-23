use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, Display, Clone, Copy, PartialEq, EnumIter)]
pub enum ENDGAME {
    Checkmate = 0,
    Stalemate = 1,
    InsufficientMaterial = 2,
    FiftyMoveRule = 3,
    ThreefoldRepetition = 4,
}
