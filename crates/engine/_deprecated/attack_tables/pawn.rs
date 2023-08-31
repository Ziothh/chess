use crate::{
    bitboard::BitBoard,
    primitives::{ChessBoard, Square, Team},
};

use super::prelude::{generate_attack_map, AttackMap};

static mut PAWN_ATTACKS: [AttackMap; Team::SIZE] = [[BitBoard::EMPTY; ChessBoard::SIZE]; Team::SIZE];

/** Generates an attack bitboard with the diagonal pawn attacks for a given team and square.
 * The team is needed because pawns can only away from their start position. */
pub fn mask_attacks(square: Square, team: Team) -> BitBoard {
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

#[rustfmt::skip]
fn generate_map() {
    Team::ALL.iter().for_each(|team| unsafe {
        PAWN_ATTACKS[team.to_index()] = generate_attack_map(
            move |square| {
                let x = mask_attacks(square, *team);
                // println!("{square}:\n{x}");
                x
            }
        );
    });
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use crate::primitives::{board::Square, team::Team};

    // #[test]
    // fn test() {
    //     generate_map();
    //     unsafe {
    //         PAWN_ATTACKS
    //             .iter()
    //             .zip(Team::ALL)
    //             .for_each(|(attack_map, team)| {
    //                 println!("\n\nTeam: {:#?}", &team);
    //
    //                 attack_map.iter().enumerate().for_each(|(i, bb)| {
    //                     // println!("{:#?}: {}\n{}", &team, Square::new(i as u8), bb)
    //                 })
    //             })
    //     }
    // }

    /// Takes in a test function and runs it for every `Team`
    fn for_team(test_fn: impl Fn(Team) -> ()) -> () {
        Team::ALL.iter().for_each(|team| test_fn(*team));
    }

    fn test_attack(square: Square, has_left: bool, has_right: bool) {
        for_team(move |team| {
            let attacking_mask = mask_attacks(square, team);

            // Should not contain the square itself
            assert_eq!(attacking_mask.has_square(square), false);

            if let Some(forward) = square.forward(team) {
                assert_eq!(
                    attacking_mask.has_square(forward.uleft()),
                    has_left
                );
                assert_eq!(
                    attacking_mask.has_square(forward.uright()),
                    has_right
                );
            }
        })
    }

    #[test]
    fn pawn_moves_A1() {
        test_attack(Square::A1, false, true)
    }
    #[test]
    fn pawn_moves_H1() {
        test_attack(Square::H1, true, false)
    }
    #[test]
    fn pawn_moves_B4() {
        test_attack(Square::B5, true, true)
    }
    #[test]
    fn pawn_moves_E4() {
        test_attack(Square::E4, true, true)
    }
    #[test]
    fn pawn_moves_G4() {
        test_attack(Square::G4, true, true)
    }
}
