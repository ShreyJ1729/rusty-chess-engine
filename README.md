# Rusty Chess Engine

A work in progress chess (and anti-chess) engine written in Rust.

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
- [x] Perft testing (node count only) - completed
- [] Replace board cloning on recursion with board make-unmake. This requires a refactor of make_move and unmake_move so to make them more light-weight. That will be done by taking the heavy validation code and putting it in generate move or validate move, so that make/unmake are purely for updating the board state.
- [] Perft testing (stratified node count among captures, promotions, checks, checkmates)
- [] Endgame Detection (checkmate, stalemate, insufficient material, repetition, 50 move rule) + perft
- [] Proper Testing Suite cfgs (unit tests, integration tests, perft tests)

## Resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Peter Keller's Blog Post](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/)
