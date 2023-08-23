use crate::{
    bitboard::BitBoard,
    core::{ChessBoard, Square, Team},
};

use super::prelude::AttackTable;

static mut PAWN_ATTACKS: AttackTable = [[BitBoard::EMPTY; ChessBoard::SIZE]; Team::SIZE];

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

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::mask_attacks;
    use crate::{
        bitboard::attack_tables::prelude::{generate_attack_map, AttackTable},
        core::{board::Square, team::Team},
    };

    #[test]
    fn pawn_moves_E4_white() {
        let pawn_square = Square::E4;
        // let pawn_bb = BitBoard::from(&pawn_square);

        let attacking_mask = mask_attacks(pawn_square, Team::White);

        // println!("{attacking_mask}");
        // println!("");
        // println!("{pawn_bb}");

        // Should not contain the square itself
        assert_eq!(attacking_mask.has_square(pawn_square), false);
        // Should have these squares to attack
        assert_eq!(attacking_mask.has_square(Square::D5), true);
        assert_eq!(attacking_mask.has_square(Square::F5), true);
    }

    #[test]
    fn general_all() {
        let attack_maps: Vec<_> = [Team::White, Team::Black]
            .iter()
            .map(|team| generate_attack_map(move |square| mask_attacks(square, *team)))
            .collect();
    }
}
