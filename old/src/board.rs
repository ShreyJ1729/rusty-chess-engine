use crate::*;
use core::panic;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub squares: [[Cell; 8]; 8],
}

impl Board {
    pub fn new(squares: [[Cell; 8]; 8]) -> Self {
        Self { squares }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::default();

        for (r, row) in fen.split('/').enumerate() {
            let mut c = 0;

            for char in row.chars() {
                if ['r', 'n', 'b', 'q', 'k', 'p', 'R', 'N', 'B', 'Q', 'K', 'P'].contains(&char) {
                    board.squares[r][c] = Cell::from_char(char);
                } else {
                    let num = char.to_digit(10).unwrap();
                    assert!(num >= 1 && num <= 8);
                    for _ in 0..num {
                        board.squares[r][c] = Cell::default();
                        c += 1;
                    }
                }
                c += 1;
            }
        }

        board
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn generate_moves(&self, turn: COLOR) -> Vec<Move> {
        let mut moves = Vec::new();
        for (r, row) in self.squares.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if cell.color == Some(turn) {
                    moves.push(self.moves_for_cell(r, c))
                }
            }
        }

        moves.into_iter().flatten().collect()
    }

    pub fn moves_for_cell(&self, row: usize, col: usize) -> Vec<Move> {
        let cell = self.squares[row][col];
        match cell.piece {
            Some(PIECE::PAWN) => self.pawn_moves(row, col),
            Some(PIECE::KNIGHT) => self.knight_moves(row, col),
            Some(PIECE::BISHOP) => self.bishop_moves(row, col),
            Some(PIECE::ROOK) => self.rook_moves(row, col),
            Some(PIECE::QUEEN) => self.queen_moves(row, col),
            Some(PIECE::KING) => self.king_moves(row, col),
            None => panic!("No piece at ({}, {})", row, col),
        }
    }

    pub fn pawn_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("pawn moves")
    }

    pub fn knight_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("knight moves")
    }

    pub fn bishop_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("bishop moves")
    }

    pub fn rook_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("rook moves")
    }

    pub fn queen_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("queen moves")
    }

    pub fn king_moves(&self, row: usize, col: usize) -> Vec<Move> {
        todo!("king moves")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new([[Cell::default(); 8]; 8])
    }
}

// TODO: implement unicode chess pieces
// https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.squares.iter().enumerate() {
            write!(f, "{} ", 8 - i)?;
            for cell in row.iter() {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")
    }
}
