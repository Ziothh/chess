use rand::{rngs::SmallRng, SeedableRng, Rng};

use crate::{bitboard::BitBoard, core::Square};
//
// TODO: use rand to get random number


fn random_bitboard<R: Rng>(rng: &mut R) -> BitBoard{
    BitBoard(rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>())
}

fn generate_magic() {
    let mut rng: SmallRng = SmallRng::seed_from_u64(0xDEADBEEF12345678);
}


fn find_magic_number(square: Square, relevant_bits: u32, bishop: u32) {
    let occupancies = [BitBoard::EMPTY; 4096];

    let attacks = [BitBoard::EMPTY; 4096];
    let used_attacks = [BitBoard::EMPTY; 4096];
}






/// NOTE: This probably wont work because I'm doing something differently. See `BitBoard.ls1b_square`
pub fn set_occupancy(index: u32, bits_in_mask: u32, mut attack_mask: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard::EMPTY;

    // Loop over the range of bits withing the attack mask
    for count in 0..bits_in_mask as u32 {
        // Get LS1B square of attack mask
        if let Some(square) = attack_mask.ls1b_square() {
            // Remove that square
            attack_mask.unset_square(square);

            // Make sure occupancy is on board
            if (index & 
                // The left shift could cause an overflow, which causes rust to panic
                // We have to do `u32.checked_shl` to get the C-like behaviour
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

    use std::io::{Write, Read};

    use crate::core::Square;
    use crate::bitboard::attack_tables;

    use super::*;

    fn pause() {
        let mut stdout = std::io::stdout();
        stdout.write(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        std::io::stdin().read(&mut [0]).unwrap();
    }


    #[test]
    fn test() {
        // let attack_mask = attack_tables::bishop::mask_attacks(Square::A1);
        // // let bb = BitBoard::new([
        // //     Square::B6,
        // //     Square::G7,
        // //     Square::E3,
        // //     Square::B2,
        // // ]);ies
        //
        // // let occupancy = set_occupancy(0, attack_mask.count_bits(), attack_mask);
        //
        // for index in 0..=4095 {
        //     let occupancy = set_occupancy(index, attack_mask.count_bits(), attack_mask);
        //
        //     println!("\n{occupancy}\nBitboard: {}\n", occupancy.to_int());
        //
        //     pause();
        // }
        //
    }
}
