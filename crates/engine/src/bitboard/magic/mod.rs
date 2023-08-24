use crate::bitboard::BitBoard;
use crate::core::Square;

/// NOTE: This probably wont work because I'm doing something differently. See `BitBoard.ls1b_square`
pub fn set_occupancy(index: u32, bits_in_mask: u8, mut attack_mask: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard::EMPTY;

    // Loop over the range of bits withing the attack mask
    for count in 0..bits_in_mask as u32 {
        // Get LS1B square of attack mask
        let square = attack_mask.ls1b_square();

        if let Some(square) = square {
            // Remove that square
            attack_mask.unset_square(square);

            // Make sure occupancy is on board
            if (index & 
                // The left shift could cause an overflow, which causes rust to panic
                // We have to do `u8.checked_shl` to get the C-like behaviour
                1u32.checked_shl(count).unwrap_or(0)
            ) != 0 {
                // Populate occupancy map
                occupancy.set_square(square);
            }
        }
    }

    return occupancy;
}

#[cfg(test)]
mod test {
    // use crate::core::{board::Square, team::Team};
    // use super::generate_attack_map;

    use crate::{bitboard::BitBoard, core::Square};
    use crate::bitboard::attack_tables;

    use super::set_occupancy;

    #[test]
    fn test() {
        let attack_mask = attack_tables::rook::mask_attacks(Square::A1);
        // let bb = BitBoard::new([
        //     Square::B6,
        //     Square::G7,
        //     Square::E3,
        //     Square::B2,
        // ]);ies

        let occupancy = set_occupancy(2, attack_mask.count_bits(), attack_mask);

        // println!("{occupancy}");
    }
}
