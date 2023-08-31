use super::prelude::{ArrayGenerator, TRANSLATIONS};
use crate::{bitboard::BitBoard, primitives::Square};

/// Generates attack maps for king pieces on every square.
pub struct KingAttacksGenerator;

impl ArrayGenerator<BitBoard> for KingAttacksGenerator {
    const NAME: &'static str = "KING_MOVES";

    fn generate_index_value(index: usize) -> BitBoard {
        let origin = Square::new(index as u8);
        let mut attacks = BitBoard::EMPTY;

        TRANSLATIONS.iter().for_each(|translation| {
            if let Some(sq) = translation(origin) {
                attacks.set_square(sq);
            }
        });

        return attacks;
    }
}
