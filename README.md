# Rusty Chess Engine

A work in progress chess engine written in Rust.

## Usage

### CLI

```bash
cargo run --release -- --mode=[perft|uci|bestmove] --depth=[depth] --fen="[fen]"
```

## Overview

## Engine Features

- [x] BitBoard representation
- [x] FEN parsing
- [x] Move generation
- - [x] Move generation for leap pieces (knight, king)
- - [x] Magic number move generation for sliding pieces (bishop, rook, queen)
- - [x] Move generation for pawns (double move, capture, and promotion)
- - [x] Move generation for castling and en passant
- [x] Move validation - is the move legal? (friendly capture or king in check)
- [] Endgame Detection (checkmate, stalemate, insufficient material, repetition, 50 move rule) + perft
- [] Perft testing
- - [x] Node count
- - [] Stratified among captures, en passants, promotions, checks, and checkmates using precalculated tables from chessprogramming wiki
- [] Unmake move
- [] Testing and metrics for each function
- - [] memory(jemalloc)
- - [] time
- - [] cpu

## Resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Peter Keller's Blog Post](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/)

Plan for refactoring

done:

- figure out new file structure and separate out files with many structs and enums
- Save generated magic numbers to cache file and load them from there if they exist.

todo:

- testing suite with precalculated stratified perft tables from chess programming wiki https://www.chessprogramming.org/Perft_Results
- Memory/time/cpu benchmarking and profiling for each function (probably some crate exists for this)
- propagate errors properly instead of just unwrapping everything. use a logging crate to log errors and debug information at different levels. Save it to a logfile.

- go through all function names for major structs and make sure they're named well. ensure consistency with variable names and shorten function bodies to be more readable when necessary.
- Some better comments/documentation/example diagrams for the really nitty grit bitshifting mechanics. Plus a nice detailed markdown page or comment description of magic bitboards.

- figure out how to turn board.rs into a module with manageable sized files

- figure out how to refactor make_move and unmake_move to be more lightweight, and move the heavy validation code to generate_move or validate_move. This will allow me to use make_move and unmake_move in the perft tests, and will also allow me to use them in the search algorithm without cloning the board every time. This has the potential to be a big performance boost.
