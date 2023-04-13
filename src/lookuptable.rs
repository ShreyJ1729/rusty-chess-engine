use std::backtrace;

use crate::*;

pub struct LookupTable {
    pub pawns: Vec<Vec<u64>>,
    pub knights: Vec<u64>,
    pub bishops: Vec<Vec<u64>>,
    pub rooks: Vec<Vec<u64>>,
    pub queens: Vec<Vec<u64>>,
    pub kings: Vec<u64>,

    pub rook_magic_numbers: Vec<u64>,

    pub move_gen: MoveGenerator,
    pub rng: rand::rngs::ThreadRng,
}

impl LookupTable {
    pub fn new() -> LookupTable {
        let table = LookupTable {
            pawns: vec![vec![0; 64]; 2],
            knights: vec![0; 64],
            bishops: vec![vec![0; 2048]; 64],
            rooks: vec![vec![0; 4096]; 64],
            queens: vec![vec![0; 4096]; 64],
            kings: vec![0; 64],

            rook_magic_numbers: vec![0; 64],

            move_gen: MoveGenerator::new(),
            rng: rand::thread_rng(),
        };

        // table.build_moves();
        table
    }

    pub fn build_moves(&mut self) {
        for square in SQUARE::iter() {
            self.build_pawn_moves(square);
            self.build_knight_moves(square);
            self.build_bishop_moves(square);
            self.build_rook_moves(square);
            self.build_queen_moves(square);
            self.build_king_moves(square);
        }
    }

    // --------------------------------------------
    // ---------- OCCUPANCY TABLES ----------------
    // --------------------------------------------

    pub fn build_rook_occupancies(square: SQUARE) -> [u64; 4096] {
        let mut rook_occupancies = [0u64; 4096];

        // iterate over all possible occupancy configurations (2^12 = 4096)
        for occupancy_index in 0..4096 {
            // println!("\nrook iterate: {}", occupancy_index);
            // println!("rook binary: {:b}", occupancy_index);

            // wrap the occupancy around the square --> first 8 bits are rank, last 8 bits are file
            let rank_bits = occupancy_index >> 6;
            let file_bits = occupancy_index & 0b111111;
            // print!("rook rank_bits: {:b}\n", rank_bits);
            // print!("rook file_bits: {:b}\n", file_bits);

            // create a bitboard for the occupancy
            let mut bb = Bitboard::new(0);

            // iterate over the square's file
            for rank in RANK::iter() {
                if (rank == RANK::Rank1) | (rank == RANK::Rank8) | (rank == square.rank()) {
                    continue;
                }

                // if the square is set in the occupancy, set the square in the occupancy bitboard
                // incrementally right-shift to check each bit in rank_bits
                let shift = rank as usize - 1;
                if file_bits >> shift & 1 == 1 {
                    let index = (rank as usize) * 8 + square.file() as usize;
                    bb.set(index);
                }
            }

            // iterate over the square's rank (reverse iter since rank_bits is read in reverse)
            for file in FILE::iter().rev() {
                if (file == FILE::FileA) | (file == FILE::FileH) | (file == square.file()) {
                    continue;
                }

                // if the square is set in the occupancy, set the square in the occupancy bitboard
                // incrementally right-shift to check each bit in file_bits
                let shift = file as usize - 1;
                if rank_bits >> shift & 1 == 1 {
                    let index = (square.rank() as usize) * 8 + file as usize;
                    bb.set(index);
                }
            }

            rook_occupancies[occupancy_index] = bb.bits();
            // println!("rook occupancy:\n{}", bb);
        }

        rook_occupancies
    }

    pub fn build_bishop_occupancies(square: SQUARE) -> [u64; 2048] {
        let mut bishop_occupancies = [0u64; 2048];

        // we have 2048 occupancies = 11 bits because in the worse case where bishop is on one of the middle squares
        // we have 11 total diagonal squares excluding edges.

        // iterate over all possible occupancy configurations (2^11 = 2048)
        for occupancy_index in 0..2048 {
            // println!("{} bishop iterate: {}", square, occupancy_index);
            // println!("{} bishop binary: {:b}", square, occupancy_index);

            let mut bb = Bitboard::new(0);
            let mut bit_index = 0;

            // ------------Northwest-Southeast------------
            // find topleft edge of square diagonal
            let mut topleft = square as usize;

            // keep going up-left until we're at rank7/fileB
            while SQUARE::from(topleft).rank() < RANK::Rank7
                && SQUARE::from(topleft).file() > FILE::FileB
            {
                topleft += 7;
            }
            // println!("{} bishop topleft: {}", square, SQUARE::from(topleft));

            let mut nw_se = topleft;

            // now we go down-right until we're at rank1/fileH, setting bits as we go
            while SQUARE::from(nw_se).rank() > RANK::Rank1
                && SQUARE::from(nw_se).file() > FILE::FileH
            {
                // if the square is set in the occupancy, set the square in the occupancy bitboard
                // incrementally right-shift to check each bit in rank_bits
                if occupancy_index >> bit_index & 1 == 1 {
                    bb.set(nw_se);
                }
                nw_se -= 7;
                bit_index += 1;
            }

            // println!(
            //     "{} bishop iterated down-left to :{}",
            //     square,
            //     SQUARE::from(nw_se)
            // );

            // ------------Northeast-Southwest------------
            // find topright edge of square diagonal
            let mut topright = square as usize;

            // keep going up-right until we're at rank7/fileH
            while SQUARE::from(topright).rank() < RANK::Rank7
                && SQUARE::from(topright).file() < FILE::FileH
            {
                topright += 9;
            }

            // println!("{} bishop topright: {}", square, SQUARE::from(topright));

            let mut ne_sw = topright;

            // now we go down-left until we're at rank1/fileA, setting bits as we go
            while SQUARE::from(ne_sw).rank() > RANK::Rank1
                && SQUARE::from(ne_sw).file() > FILE::FileA
            {
                // if the square is set in the occupancy, set the square in the occupancy bitboard
                // incrementally right-shift to check each bit in rank_bits
                if occupancy_index >> bit_index & 1 == 1 {
                    bb.set(ne_sw);
                }
                ne_sw -= 9;
                bit_index += 1;
            }

            bishop_occupancies[occupancy_index] = bb.bits();
            // print it
            // println!("bishop occupancy:\n{}", bb);
        }

        bishop_occupancies
    }

    // --------------------------------------------
    // ------------ MAGIC NUMBERS -----------------
    // --------------------------------------------

    pub fn generate_magic_number_candidate(&mut self) -> u64 {
        self.rng.gen::<u64>() & self.rng.gen::<u64>() & self.rng.gen::<u64>()
    }

    pub fn validate_bishop_magic_number(
        &mut self,
        magic_number: u64,
        bishop_occupancies: &[u64],
        square: SQUARE,
    ) -> bool {
        for occupancy_index in 0..2048 {
            let occupancy = bishop_occupancies[occupancy_index];
            // println!("occupancy\n: {}", Bitboard::new(occupancy));
            let moves = self
                .move_gen
                .generate_bishop_moves(square, Bitboard::new(occupancy));

            // compute this hash: hash = (occupancy * magic_number_candidate) >> (64 - 11)
            let hash = (occupancy.wrapping_mul(magic_number) >> (64 - 11)) as usize;

            // check if collision occurs (value is already set and is not equal to the moves we just computed)
            let value_at_hash = self.bishops[square as usize][hash];
            let collision = value_at_hash != 0 && value_at_hash != moves.bits();
            if collision {
                // if a collision occurs then clear array, pick a new magic number and try again
                // println!("candidate failed: {}", magic_number_candidate);
                self.bishops[square as usize] = vec![0; 2048];
                return false;
            }

            // if no collision, set the value at the hash to the moves
            self.bishops[square as usize][hash] = moves.bits();
        }

        true
    }

    pub fn validate_rook_magic_number(
        &mut self,
        magic_number: u64,
        rook_occupancies: &[u64],
        square: SQUARE,
    ) -> bool {
        for occupancy_index in 0..4096 {
            let occupancy = rook_occupancies[occupancy_index];
            // println!("occupancy\n: {}", Bitboard::new(occupancy));
            let moves = self
                .move_gen
                .generate_rook_moves(square, Bitboard::new(occupancy));

            // compute this hash: hash = (occupancy * magic_number_candidate) >> (64 - 12)
            let hash = (occupancy.wrapping_mul(magic_number) >> (64 - 12)) as usize;

            // check if collision occurs (value is already set and is not equal to the moves we just computed)
            let value_at_hash = self.rooks[square as usize][hash];
            let collision = value_at_hash != 0 && value_at_hash != moves.bits();

            if collision {
                // if a collision occurs then clear array, pick a new magic number and try again
                // println!("candidate failed: {}", magic_number_candidate);
                self.rooks[square as usize] = vec![0; 4096];
                return false;
            }

            // if no collision, set the value at the hash to the moves we just computed
            self.rooks[square as usize][hash] = moves.bits();
        }

        true
    }

    // --------------------------------------------
    // ------------- LEAPING MOVES ----------------
    // --------------------------------------------

    pub fn build_pawn_moves(&mut self, square: SQUARE) {
        for color in COLOR::iter() {
            let moves = self.move_gen.generate_pawn_moves(square, color);
            self.pawns[color as usize][square as usize] = moves.bits();
        }
    }

    pub fn build_king_moves(&mut self, square: SQUARE) {
        let moves = self.move_gen.generate_king_moves(square);
        self.kings[square as usize] = moves.bits();
    }

    pub fn build_knight_moves(&mut self, square: SQUARE) {
        let moves = self.move_gen.generate_knight_moves(square);
        self.knights[square as usize] = moves.bits();
    }

    // --------------------------------------------
    // ------------- SLIDING MOVES ----------------
    // --------------------------------------------

    pub fn build_bishop_moves(&mut self, square: SQUARE) {
        let bishop_occupancies = LookupTable::build_bishop_occupancies(square);

        // we compute magic numbers for each square
        let mut magic_found = false;
        let mut magic_number = 0;

        while !magic_found {
            magic_number = self.generate_magic_number_candidate();
            // validationg simultaneously validates the magic number and computing the moves for each occupancy
            magic_found =
                self.validate_bishop_magic_number(magic_number, &bishop_occupancies, square)
        }

        println!(
            "{} bishop moves built - using {} for hash key",
            square, magic_number
        );
        self.rook_magic_numbers[square as usize] = magic_number;
    }

    pub fn build_rook_moves(&mut self, square: SQUARE) {
        let rook_occupancies = LookupTable::build_rook_occupancies(square);

        // we compute magic numbers for each square
        let mut magic_found = false;
        let mut magic_number = 0;

        while !magic_found {
            magic_number = self.generate_magic_number_candidate();
            // validationg simultaneously validates the magic number and computing the moves for each occupancy
            magic_found = self.validate_rook_magic_number(magic_number, &rook_occupancies, square)
        }

        println!(
            "{} rook moves built - using {} for hash key",
            square, magic_number
        );
        self.rook_magic_numbers[square as usize] = magic_number;

        // a little sanity check to make sure the magic number is correct
        // ------------------------------------------------------------
        // now use the magic number to compute the moves for each occupancy configuration, printing out
        // the occupancy and moves for each configuration

        for occupancy_index in 0..4096 {
            let occupancy = rook_occupancies[occupancy_index];
            let moves = self
                .move_gen
                .generate_rook_moves(square, Bitboard::new(occupancy));

            let hash = (occupancy.wrapping_mul(self.rook_magic_numbers[square as usize])
                >> (64 - 12)) as usize;
            let predicted = self.rooks[square as usize][hash];

            // println!("occupancy:\n{}", Bitboard::new(occupancy));
            // println!("predicted:\n{}", Bitboard::new(predicted));
            // println!("moves:\n{}", moves);
            // println!("hash: {}", hash);
            // println!("---------------------");
            assert!(predicted == moves.bits());
        }
        // ------------------------------------------------------------
    }

    pub fn build_queen_moves(&mut self, square: SQUARE) {}
}
