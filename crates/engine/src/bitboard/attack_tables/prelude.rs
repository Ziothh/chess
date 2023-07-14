use crate::{
    bitboard::BitBoard,
    core::{board::Square, NUM_SQUARES},
};

pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
/** Translations that move a square by 1.
 * TRANSLATIONS[..4] are 1D 
 * TRANSLATIONS[4..] are 2D (diagonal) */
pub const TRANSLATIONS: [SquareTranslation; 8] = [
    // Horizontal + vertical
    |origin| origin.up(),    // N
    |origin| origin.right(), // E
    |origin| origin.down(),  // S
    |origin| origin.left(),  // W
    // Diagonal
    |origin| origin.up().and_then(|sq| sq.right()), // NE
    |origin| origin.up().and_then(|sq| sq.left()),  // NW
    |origin| origin.down().and_then(|sq| sq.right()), // SE
    |origin| origin.down().and_then(|sq| sq.left()), // SW
];

