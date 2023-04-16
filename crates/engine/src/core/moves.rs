use super::{board::Square, instructions::CheckKind, piece::ChessPieceVariant};

/// Data structure representing a single move.
#[derive(Debug, PartialEq)]
pub struct Move {
    /// The original board square index
    pub origin: Square,
    /// The destination board square index
    pub destination: Square,
    /// If this piece takes another piece
    pub takes: bool,
    /// If this move checks the King
    pub checks: Option<CheckKind>,
    /// The piece the pawn promotes to
    pub promotion: Option<ChessPieceVariant>,
    /// The moving piece
    pub piece: ChessPieceVariant,
}

impl Move {
    pub fn new(piece: ChessPieceVariant, origin: Square, destination: Square) -> Self {
      Self::new_with_promotion(piece, origin, destination, None)
    }
    pub fn new_with_promotion(piece: ChessPieceVariant, origin: Square, destination: Square, promotion: Option<ChessPieceVariant>) -> Self {
        Self {
            piece,
            origin,
            destination,
            promotion,
            checks: None,
            takes: false,
        }
    }
}
