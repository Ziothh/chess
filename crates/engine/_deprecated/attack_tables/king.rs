use crate::{bitboard::BitBoard, primitives::Square};

use super::prelude::TRANSLATIONS;

/** Generates an attack bitboard for a knight on a certain position */
pub fn mask_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    TRANSLATIONS.iter().for_each(|translation| {
        if let Some(sq) = translation(square) {
            attacks.set_square(sq);
        }
    });

    return attacks;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::mask_attacks;
    use crate::primitives::board::Square;

    #[test]
    fn attacks_E4() {
        assert_eq!(
            mask_attacks(Square::E4).to_string(),
            [
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . x x x . .",
                ". . . x . x . .", // 4
                ". . . x x x . .",
                ". . . . . . . .",
                ". . . . . . . .",
                //       e
            ]
            .join("\n")
        );
    }
    #[test]
    fn attacks_H1() {
        assert_eq!(
            mask_attacks(Square::H1).to_string(),
            [
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . x x",
                ". . . . . . x .",
            ]
            .join("\n")
        );
    }
    #[test]
    fn attacks_A8() {
        assert_eq!(
            mask_attacks(Square::A8).to_string(),
            [
                ". x . . . . . .",
                "x x . . . . . .", // 7
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                //a
            ]
            .join("\n")
        );
    }
}
