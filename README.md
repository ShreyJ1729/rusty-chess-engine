# Rusty Chess Engine

A work in progress chess (and anti-chess) engine written in Rust, as a learning exercise.

## Usage

### CLI

```bash
cargo run --release -- -mode=[perft|uci|bestmove] -depth=[depth] -fen="[fen]"
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
- [x] Perft testing (node count only) - completed at 4M nodes/sec
- [] CLI interface
- [] Perft Progress Bar for each submove
- [] Proper Testing Suite cfgs (unit tests, integration tests, perft tests)
- [] Endgame Detection (checkmate, stalemate, insufficient material, repetition, 50 move rule)
- [] Perft testing (stratified node count among captures, promotions, checks, checkmates)
- [] Move-generation performance optimization
- - [] Single-threaded
- - [] Multi-threaded

## Resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Peter Keller's Blog Post](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/)
