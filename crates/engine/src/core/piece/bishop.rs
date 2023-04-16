use crate::core::{
    board::Square,
    moves::Move,
    piece::{ChessPieceVariant, PieceType, SlidingPiece},
    team::Team,
};

pub struct BishopType;

impl PieceType for BishopType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Bishop;

    fn pseudo_legal_moves(position: Square, _team: Team) -> Vec<Move> {
        return Self::generate_sliding_destionations(position)
            .into_iter()
            .map(|destination| Move::new(Self::PIECE_VARIANT, position, destination))
            .collect();
    }
}

impl SlidingPiece for BishopType {
    const TRANSLATIONS: &'static [fn(origin: Square) -> Option<Square>] = &[
        |x| x.right()?.up(),
        |x| x.right()?.down(),
        |x| x.left()?.up(),
        |x| x.left()?.down(),
    ];
}
