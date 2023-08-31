mod rook;
mod bishop;

use crate::{bitboard::BitBoard, primitives::piece::SlidingDirection};

use super::prelude::ArrayGenerator;

/// Generator for the rays of rooks and bishops.
///
/// ```ignore
/// const RAYS = [
///     ROOK_RAYS,
///     BISHOP_RAYS
/// ]
/// ```
pub struct RaysGenerator;
impl ArrayGenerator<[BitBoard; 64], 2 /* SlidingDirection::SIZE */> for RaysGenerator {
    const NAME: &'static str = "RAYS";

    fn generate_index_value(_index: usize) -> [BitBoard; 64] {
        unreachable!()
    }

    fn generate_array() -> [[BitBoard; 64]; SlidingDirection::SIZE] {
        return [
            rook::RookRaysGenerator::generate_array(),
            bishop::BishopRaysGenerator::generate_array(),
        ]
    }
}


