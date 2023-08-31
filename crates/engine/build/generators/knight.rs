use super::prelude::{ArrayGenerator, SquareTranslation};
use crate::{bitboard::BitBoard, primitives::Square};

const KNIGHT_TRANSLATIONS: [SquareTranslation; 8] = [
    |origin: Square| origin.translate(2, 1),
    |origin: Square| origin.translate(2, -1),
    |origin: Square| origin.translate(-2, 1),
    |origin: Square| origin.translate(-2, -1),
    |origin: Square| origin.translate(1, 2),
    |origin: Square| origin.translate(-1, 2),
    |origin: Square| origin.translate(1, -2),
    |origin: Square| origin.translate(-1, -2),
];

/// Generates attack maps for knight pieces on every square.
pub struct KnightAttacksGenerator;

impl ArrayGenerator<BitBoard> for KnightAttacksGenerator {
    const NAME: &'static str = "KNIGHT_MOVES";

    fn generate_index_value(index: usize) -> BitBoard {
        let origin = Square::new(index as u8);
        let mut attacks = BitBoard::EMPTY;

        KNIGHT_TRANSLATIONS.iter().for_each(|translation| {
            if let Some(sq) = translation(origin) {
                attacks.set_square(sq);
            }
        });

        return attacks;
    }
}
