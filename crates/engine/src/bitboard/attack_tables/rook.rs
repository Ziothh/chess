use crate::{bitboard::BitBoard, core::Square};

use super::prelude::TRANSLATIONS;

pub fn mask_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    for translation in TRANSLATIONS[..4].iter() {
        let mut current_square = square;

        while let Some(sq) = translation(current_square) {
            attacks.set_square(sq);
            current_square = sq;
        }
    }

    return attacks;
}

pub fn mask_attacks_on_the_fly(square: Square, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    for translation in TRANSLATIONS[..4].iter() {
        let mut current_square = square;

        while let Some(sq) = translation(current_square) {
            attacks.set_square(sq);

            if blockers.has_square(sq) {
                break;
            };

            current_square = sq;
        }
    }

    return attacks;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::mask_attacks;
    use crate::{bitboard::BitBoard, core::board::Square};

    #[test]
    fn attacks_E4() {
        assert_eq!(
            mask_attacks(Square::E4).to_string(),
            [
                ". . . . x . . .",
                ". . . . x . . .",
                ". . . . x . . .",
                ". . . . x . . .",
                "x x x x . x x x", // 4
                ". . . . x . . .",
                ". . . . x . . .",
                ". . . . x . . .",
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
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                "x x x x x x x .",
            ]
            .join("\n")
        );
    }
    #[test]
    fn attacks_A8() {
        assert_eq!(
            mask_attacks(Square::A8).to_string(),
            [
                ". x x x x x x x",
                "x . . . . . . .", // 7
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                //a
            ]
            .join("\n")
        );
    }

    #[test]
    fn attacks_on_the_fly_D4() {
        let blockers = BitBoard::new([
            // Squares with pieces
            Square::D7,
            Square::D2,
            Square::B4,
            Square::G4,
        ]);
        assert_eq!(
            blockers.to_string(),
            [
                ". . . . . . . .",
                ". . . x . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". x . . . . x .",
                ". . . . . . . .",
                ". . . x . . . .",
                ". . . . . . . .",
            ]
            .join("\n")
        );

        let attack_map = super::mask_attacks_on_the_fly(Square::D4, blockers);
        assert_eq!(
            attack_map.to_string(),
            [
                ". . . . . . . .",
                ". . . x . . . .",
                ". . . x . . . .",
                ". . . x . . . .",
                ". x x . x x x .",
                ". . . x . . . .",
                ". . . x . . . .",
                ". . . . . . . .",
            ]
            .join("\n")
        );
    }
}
