mod prelude;

pub mod bishop;
mod king;
mod knight;
mod pawn;
pub mod rook;

use crate::core::{board::Square, team::Team, ChessBoard};

use crate::bitboard::BitBoard;

type AttackMap = [BitBoard; ChessBoard::SIZE];
pub fn generate_attack_map(bitboard_generator: fn(square: Square) -> BitBoard) -> AttackMap {
    let mut attack_map: AttackMap = [BitBoard::EMPTY; ChessBoard::SIZE];

    attack_map
        .iter_mut()
        .enumerate()
        .for_each(|(index, bitboard)| {
            *bitboard = bitboard_generator(Square::new(index as u8));
        });

    return attack_map;
}

fn _test() {
    let _pawn_attacks = [
        generate_attack_map(|square| pawn::mask_attacks(square, Team::White)),
        generate_attack_map(|square| pawn::mask_attacks(square, Team::Black)),
    ];

    let _knight_attacks = generate_attack_map(knight::mask_attacks);
}

#[cfg(test)]
mod test {
    // use crate::core::{board::Square, team::Team};
    // use super::generate_attack_map;

    use crate::{bitboard::BitBoard, core::Square};

    #[test]
    fn test() {
        // let attack_maps = generate_attack_map(super::rook::mask_attacks);
        //
        // for x in attack_maps.iter() {
        //     println!("{x}\n");
        // }

        let blockers = BitBoard::new([
            // Squares with pieces
            Square::B6,
            Square::G7,
            Square::E3,
            Square::B2,
        ]);

        let x = super::bishop::mask_attacks_on_the_fly(Square::D4, blockers.clone());

        // println!("{}\n", blockers.clone() | x.clone());

        // println!("{blockers}\n");
        // println!("{x}");
    }
}
