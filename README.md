# Rusty Chess Engine

A work in progress chess engine written in Rust.

## Usage

### CLI

```bash
cargo run --release -- --max_depth=[depth] --fen="[fen]"
```

## Overview

## Engine Features

- [x] BitBoard representation
- [x] FEN parsing
- [x] Move generation with magic bitboards
- [] Perft testing suite (all left is checkmate detection)
- [] Unmake move
- [] Testing and metrics for each function
- - [] memory(jemalloc)
- - [] time
- - [] cpu
- CLI using UCI protocol
- GUI using bevy game engine

## Resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Peter Keller's Blog Post](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/)

Plan for refactoring

todo:

- Memory/time/cpu benchmarking and profiling for each function (probably some crate exists for this)
- propagate errors properly instead of just unwrapping everything. use a logging crate to log errors and debug information at different levels. Save it to a logfile.

- go through all function names for major structs and make sure they're named well. ensure consistency with variable names and shorten function bodies to be more readable when necessary.
- Some better comments/documentation/example diagrams for the really nitty grit bitshifting mechanics. Plus a nice detailed markdown page or comment description of magic bitboards.

- figure out how to turn board.rs into a module with manageable sized files

- figure out how to refactor make_move and unmake_move to be more lightweight, and move the heavy validation code to generate_move or validate_move. This will allow me to use make_move and unmake_move in the perft tests, and will also allow me to use them in the search algorithm without cloning the board every time. This has the potential to be a big performance boost.
