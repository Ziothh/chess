use crate::{bitboard::BitBoard, primitives::board::Square};

use super::prelude::SquareTranslation;

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

/** Generates an attack bitboard for a knight on a certain position */
pub fn mask_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    KNIGHT_TRANSLATIONS.iter().for_each(|translation| {
        if let Some(sq) = translation(square) {
            attacks.set_square(sq);
        }
    });

    return attacks;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::primitives::board::Square;
    use super::mask_attacks;

    #[test]
    fn knight_moves_E4() {
        assert_eq!(
            mask_attacks(Square::E4).to_string(),
            [
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . x . x . .",
                ". . x . . . x .",
                ". . . . . . . .", // 4
                ". . x . . . x .",
                ". . . x . x . .",
                ". . . . . . . .",
                //       e
            ]
            .join("\n")
        );
    }
    #[test]
    fn knight_moves_G2() {
        assert_eq!(
            mask_attacks(Square::G2).to_string(),
            [
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . x . x",
                ". . . . x . . .",
                ". . . . . . . .", // 2
                ". . . . x . . .",
                //           g
            ]
            .join("\n")
        );
    }
    #[test]
    fn knight_moves_B7() {
        assert_eq!(
            mask_attacks(Square::B7).to_string(),
            [
                ". . . x . . . .",
                ". . . . . . . .", // 7
                ". . . x . . . .",
                "x . x . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                // b
            ]
            .join("\n")
        );
    }
}
