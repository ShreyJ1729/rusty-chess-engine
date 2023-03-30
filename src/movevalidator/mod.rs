fn validate_move(&self, moves: Vec<Move>, color: COLOR) -> Vec<Move> {
    // filters out moves where target is friendly piece
    let moves = moves
        .into_iter()
        .filter(|move_| {
            let piece = self.board.piece_at(move_.target);
            !piece.is_color(color)
        })
        .collect();
    moves
}
