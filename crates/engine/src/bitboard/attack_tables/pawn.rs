use crate::{
    bitboard::BitBoard,
    core::{Square, Team},
};

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
    use crate::core::{board::Square, team::Team};
    use super::mask_attacks;

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
}