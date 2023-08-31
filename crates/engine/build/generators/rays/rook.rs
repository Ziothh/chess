use crate::{bitboard::BitBoard, primitives::Square};
use super::super::prelude::{square_with_i8_coords, ArrayGenerator};

/// Generates rays for rook pieces on every square.
pub struct RookRaysGenerator;

impl ArrayGenerator<BitBoard> for RookRaysGenerator {
    const NAME: &'static str = "ROOK_RAYS";

    fn generate_index_value(index: usize) -> BitBoard {
        let origin = Square::new(index as u8);
        let (origin, origin_file, origin_rank) = square_with_i8_coords(&origin);

        return Square::ALL
            .iter()
            .filter(|dest| {
                let (dest, dest_file, dest_rank) = square_with_i8_coords(dest);

                return (origin_file == dest_file || origin_rank == dest_rank) && *origin != *dest
            })
            .fold(BitBoard::EMPTY, |bb, square| bb | BitBoard::from(square));
    }
}
