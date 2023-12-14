use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{Piece, PieceType},
    team::Team,
};

pub struct KingType;

impl PieceType for KingType {
    const PIECE_VARIANT: Piece = Piece::Bishop;

    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move> {
        let moves: Vec<_> = Vec::new();

        return moves;
        // todo!()
    }
}
