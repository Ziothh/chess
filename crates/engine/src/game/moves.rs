use crate::primitives::{board::Square, piece::Piece};

/// Data structure representing a single move.
#[derive(Debug, rspc::Type, serde::Serialize, PartialEq)]
pub struct Move {
    /// The original board square index
    pub origin: Square,
    /// The destination board square index
    pub destination: Square,
    /// The piece the pawn promotes to
    pub promotion: Option<Piece>,
}

// pub struct Move {
//     /// The original board square index
//     pub origin: Square,
//     /// The destination board square index
//     pub destination: Square,
//     /// If this piece takes another piece
//     pub takes: bool,
//     /// If this move checks the King
//     pub checks: Option<CheckKind>,
//     /// The piece the pawn promotes to
//     pub promotion: Option<Piece>,
//     /// The moving piece
//     pub piece: Piece,
// }
//
// impl Move {
//     pub fn new(piece: Piece, origin: Square, destination: Square) -> Self {
//       Self::new_with_promotion(piece, origin, destination, None)
//     }
//     pub fn new_with_promotion(piece: Piece, origin: Square, destination: Square, promotion: Option<Piece>) -> Self {
//         Self {
//             piece,
//             origin,
//             destination,
//             promotion,
//             checks: None,
//             takes: false,
//         }
//     }
// }
