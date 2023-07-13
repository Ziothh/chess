use crate::core::{
    board::{Square, NUM_SQUARES},
    team::{self, Team, NUM_TEAMS},
};

use super::BitBoard;

type AttackMap = [BitBoard; NUM_SQUARES];

pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
const TRANSLATIONS: [SquareTranslation; 8] = [
    // Horizontal + vertical
    |origin| origin.up(),    // N
    |origin| origin.right(), // E
    |origin| origin.down(),  // S
    |origin| origin.left(),  // W
    // Diagonal
    |origin| origin.up().and_then(|sq| sq.right()), // NE
    |origin| origin.up().and_then(|sq| sq.left()),  // NW
    |origin| origin.down().and_then(|sq| sq.right()), // SE
    |origin| origin.down().and_then(|sq| sq.left()), // SW
];
const STRAIGHT_TRANSLATIONS: &[SquareTranslation] = &TRANSLATIONS[0..4];
const DIAGONAL_TRANSLATIONS: &[SquareTranslation] = &TRANSLATIONS[4..];

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

/** Generates an attack bitboard with the diagonal pawn attacks for a given team and square */
fn mask_pawn_attack(square: Square, team: Team) -> BitBoard {
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
}

/** */
fn mask_knight_attack(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    return attacks;
}

// fn generate_attack_map(translations: &[SquareTranslation], repeat_translations: bool) -> AttackMap {
//     let mut attack_map: AttackMap = [BitBoard::EMPTY; NUM_SQUARES];
//
//     attack_map
//         .iter_mut()
//         .enumerate()
//         .for_each(|(square_index, bitboard)| {
//             let square = Square::new(square_index as u8);
//
//             // *bitboard = bitboard_generator();
//             translations.iter().for_each(|tr| loop {
//                 let current_squre = square;
//                 // todo improve this with a while loop
//
//                 if let Some(new_square) = tr(square) {
//                     bitboard.set_square(new_square);
//
//                     if repeat_translations == false {
//                         break;
//                     }
//                 }
//             })
//         });
//
//     return attack_map;
// }

fn generate_attack_map(bitboard_generator: fn(square: Square) -> BitBoard) -> AttackMap {
    let mut attack_map: AttackMap = [BitBoard::EMPTY; NUM_SQUARES];

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
        generate_attack_map(|square| mask_pawn_attack(square, Team::White)),
        generate_attack_map(|square| mask_pawn_attack(square, Team::Black)),
    ];

    let _knight_attacks = generate_attack_map(mask_knight_attack);
}

#[cfg(test)]
mod test {
    use crate::{
        bitboard::{
            attack_tables::{generate_attack_map, mask_pawn_attack},
            BitBoard,
        },
        core::{board::Square, team::Team},
    };

    #[test]
    fn pawn_moves_e4_white() {
        let pawn_square = Square::E4;
        // let pawn_bb = BitBoard::from(&pawn_square);

        let attacking_mask = mask_pawn_attack(pawn_square, Team::White);

        // println!("{attacking_mask}");
        // println!("");
        // println!("{pawn_bb}");

        // Should not contain the square itself
        assert_eq!(attacking_mask.has_square(&pawn_square), false);
        // Should have these squares to attack
        assert_eq!(attacking_mask.has_square(&Square::D5), true);
        assert_eq!(attacking_mask.has_square(&Square::F5), true);
    }
}
