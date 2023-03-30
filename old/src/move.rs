use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promotion: Option<PIECE>,
    pub capture: Option<PIECE>,
}

impl Move {
    pub fn new(
        from: (usize, usize),
        to: (usize, usize),
        promotion: Option<PIECE>,
        capture: Option<PIECE>,
    ) -> Self {
        assert!((0..8).contains(&from.0));
        assert!((0..8).contains(&from.1));
        assert!((0..8).contains(&to.0));
        assert!((0..8).contains(&to.1));

        match promotion {
            Some(PIECE::PAWN) => panic!("Cannot promote to pawn"),
            Some(PIECE::KING) => panic!("Cannot promote to king"),
            None => Self {
                from,
                to,
                promotion: None,
                capture: None,
            },
            _ => Self {
                from,
                to,
                promotion,
                capture,
            },
        }
    }
}
