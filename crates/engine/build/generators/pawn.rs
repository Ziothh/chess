use crate::{
    bitboard::BitBoard,
    primitives::{Square, Team, Rank, File},
};

use super::prelude::{ArrayGenerator, ValueGenerator};

/// Generates moves for pawn pieces on every square.
pub struct PawnMovesGenerator;
impl ArrayGenerator<[BitBoard; Square::AMOUNT], 2 /* Team::SIZE */> for PawnMovesGenerator {
    const NAME: &'static str = "PAWN_MOVES";

    // For every team
    fn generate_index_value(index: usize) -> [BitBoard; Square::AMOUNT] {
        let team = Team::try_from_index(index).expect("index to be 0 or 1");

        return (0..Square::AMOUNT as u8)
            .into_iter()
            .map(Square::new)
            .map(|square| {
                let mut moves = BitBoard::EMPTY;

                if square.get_rank() == team.get_nth_rank(2) {
                    moves.set_square(square.uforward(team));
                    moves.set_square(square.uforward(team).uforward(team));
                } else {
                    moves.set_maybe_square(square.forward(team));
                }

                return moves;
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
}

/// Generates attack maps for pawn pieces on every square.
pub struct PawnAttacksGenerator;
impl ArrayGenerator<[BitBoard; Square::AMOUNT], 2 /* Team::SIZE */> for PawnAttacksGenerator {
    const NAME: &'static str = "PAWN_ATTACKS";

    // For every team
    fn generate_index_value(index: usize) -> [BitBoard; Square::AMOUNT] {
        let team = Team::try_from_index(index).expect("index to be 0 or 1");

        return (0..Square::AMOUNT as u8)
            .into_iter()
            .map(Square::new)
            .map(|square| {
                let mut attacks = BitBoard::EMPTY;

                if let Some(forward) = square.forward(team) {
                    if let Some(sq) = forward.left() {
                        attacks.set_square(sq);
                    }
                    if let Some(sq) = forward.right() {
                        attacks.set_square(sq);
                    }
                }

                return attacks;
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
}

pub struct PawnDoubleMoveOriginsGenerator;
impl ValueGenerator<BitBoard> for PawnDoubleMoveOriginsGenerator {
    const NAME: &'static str = "PAWN_ORIGIN_DOUBLE_MOVES";

    fn generate_value() -> BitBoard {
        let mut result = BitBoard::EMPTY;
        for rank in [Rank::Second, Rank::Seventh].iter() {
            for file in File::ALL.iter() {
                result.set_square(Square::make_square(*file, *rank));
            }
        }
        return result;
    }
}


pub struct PawnDoubleMoveDestinationsGenerator;
impl ValueGenerator<BitBoard> for PawnDoubleMoveDestinationsGenerator {
    const NAME: &'static str = "PAWN_DEST_DOUBLE_MOVES";

    fn generate_value() -> BitBoard {
        let mut result = BitBoard::EMPTY;
        for rank in [Rank::Fourth, Rank::Fifth].iter() {
            for file in File::ALL.iter() {
                result.set_square(Square::make_square(*file, *rank));
            }
        }
        return result;
    }
}
