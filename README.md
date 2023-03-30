# Rusty Chess Engine

A work in progress chess engine written in Rust, as a learning exercise.

## Overview

struct Game

- board: [Piece; 16]
- turn: Color

struct Piece

- color: Color
- piece_type: PieceType
- square: Square

enum Color (White or Black)

enum PieceType (Pawn, Knight, Bishop, Rook, Queen, King)

enum Square (A1, A2, ..., H8)

## Task List

- [x] BitBoard representation
- [] Move generation
- - [] Move generation for leap pieces (knight, king)
- - [] Move generation for sliding pieces (bishop, rook, queen)
- - [] Move generation for pawns (double move, capture, and promotion)
- - [] Move generation for castling and en passant
- [] Move validation
- [] Do perft tests to ensure move generation is correct

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
