# Rusty Chess Engine

A work in progress chess (and anti-chess) engine written in Rust, as a learning exercise.

## Usage

### CLI

```bash
cargo run --release -- -mode=[perft|uci] -depth=[depth] -fen="[fen]"
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
- [] Proper Testing Suite cfgs (unit tests, integration tests, perft tests)
- [] Endgame Detection (checkmate, stalemate, insufficient material, repetition, 50 move rule)
- [] Perft testing (stratified node count)
- [] Move-generation performance optimization
- - [] Single-threaded
- - [] Multi-threaded

## Resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Peter Keller's Blog Post](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/)

## FAQ

### Why Rust?

I've been wanting to learn Rust for a while, especially learning more about safe memory management and concurrency. A chess engine is a nice problem that deals with these concepts at a reasonable scale of complexity.

### Why is it called "Rusty Chess Engine"?

Knowing my programming skills and the state of the project, it's a pretty accurate name.

### Why is there no Human vs. Human mode?

I'm lazy. Go play on chess.com or something.
