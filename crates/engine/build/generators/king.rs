use super::prelude::{ArrayGenerator, ValueGenerator, TRANSLATIONS};
use crate::{
    bitboard::BitBoard,
    primitives::{File, Square, Team},
};

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

pub struct KingsideCastleSquaresGenerator;
impl ArrayGenerator<BitBoard, 2 /* Team::SIZE */> for KingsideCastleSquaresGenerator {
    const NAME: &'static str = "KINGSIDE_CASTLE_SQUARES";

    fn generate_index_value(index: usize) -> BitBoard {
        let back_rank = Team::try_from_index(index).unwrap().get_nth_rank(1);

        return BitBoard::new([
            Square::make_square(File::F, back_rank),
            Square::make_square(File::G, back_rank),
        ]);
    }
}

pub struct QueensideCastleSquaresGenerator;
impl ArrayGenerator<BitBoard, 2 /* Team::SIZE */> for QueensideCastleSquaresGenerator {
    const NAME: &'static str = "QUEENSIDE_CASTLE_SQUARES";

    fn generate_index_value(index: usize) -> BitBoard {
        let back_rank = Team::try_from_index(index).unwrap().get_nth_rank(1);

        return BitBoard::new([
            Square::make_square(File::B, back_rank),
            Square::make_square(File::C, back_rank),
            Square::make_square(File::D, back_rank),
        ]);
    }
}

pub struct CastleMovesGenerator;
impl ValueGenerator<BitBoard> for CastleMovesGenerator {
    const NAME: &'static str = "CASTLE_MOVES";

    fn generate_value() -> BitBoard {
        BitBoard::new([
            Square::C1,
            Square::C8,
            Square::E1,
            Square::E1,
            Square::E8,
            Square::G1,
            Square::G8,
        ])
    }
}
