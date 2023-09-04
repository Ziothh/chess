use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{Piece, PieceType},
    team::Team,
};

use super::{PieceMovement, SquareTranslation};

pub struct BishopType;

impl PieceType for BishopType {
    const PIECE_VARIANT: Piece = Piece::Bishop;

    fn pseudo_legal_moves(position: Square, _team: Team) -> Vec<Move> {
        return Self::generate_sliding_destionations(position)
            .into_iter()
            .map(|destination| Move::new(Self::PIECE_VARIANT, position, destination))
            .collect();
    }
}

impl PieceMovement for BishopType {
    const IS_SLIDING: bool = true;
    const TRANSLATIONS: &'static [SquareTranslation] = &[
        // Diagonal
        Self::NE,
        Self::NW,
        Self::SE,
        Self::SW,
    ];
}
