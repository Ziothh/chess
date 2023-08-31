mod bishop;
mod rook;

use crate::{
    bitboard::BitBoard,
    primitives::{piece::SlidingDirection, Square},
};

use super::prelude::ArrayGenerator;

static mut RAYS: [[BitBoard; Square::AMOUNT]; SlidingDirection::SIZE] =
    [[BitBoard::EMPTY; Square::AMOUNT]; SlidingDirection::SIZE];

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

    fn generate_index_value(_index: usize) -> [BitBoard; Square::AMOUNT] {
        unreachable!()
    }

    fn generate_array() -> [[BitBoard; Square::AMOUNT]; SlidingDirection::SIZE] {
        unsafe {
            RAYS = [
                rook::RookRaysGenerator::generate_array(),
                bishop::BishopRaysGenerator::generate_array(),
            ];

            return RAYS;
        }
    }
}

impl RaysGenerator {
    /// NOTE: only use this if the `RAYS` static array has been initialised by `Self::generate_array()`
    pub fn get_rays(square: Square, sliding_direction: SlidingDirection) -> BitBoard {
        unsafe { RAYS[sliding_direction.to_index()][square.to_index()] }
    }
}
