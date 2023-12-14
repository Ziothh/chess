use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{Piece, PieceType, SlidingPiece},
    team::Team,
};

pub struct RookType;

impl PieceType for RookType {
    const PIECE_VARIANT: Piece = Piece::Rook;

    fn pseudo_legal_moves(position: Square, _team: Team) -> Vec<Move> {
        return Self::generate_sliding_destionations(position)
            .into_iter()
            .map(|destination| Move::new(Self::PIECE_VARIANT, position, destination))
            .collect();
    }
}

impl SlidingPiece for RookType {
    const TRANSLATIONS: &'static [fn(origin: Square) -> Option<Square>] = &[
        //
        |x| x.left(),
        |x| x.right(),
        |x| x.up(),
        |x| x.down(),
    ];
}
