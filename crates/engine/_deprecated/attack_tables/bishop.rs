use crate::{bitboard::BitBoard, primitives::Square};

use super::prelude::TRANSLATIONS;

#[rustfmt::skip]
/// The max amount of bits set to 1 in the attack mask (by mask_attack()) for every square
const RELEVANT_BITS: [u32; BitBoard::SIZE] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

pub fn mask_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    for translation in TRANSLATIONS[4..].iter() {
        let mut current_square = square;

        while let Some(sq) = translation(current_square) {
            attacks.set_square(sq);
            current_square = sq;
        }
    }

    return attacks & BitBoard::NOT_BORDERS;
}

/// Like `bishop::mask_attacks`, but the rays stop as soon as they encounter any bits set in `blockers`
pub fn mask_attacks_on_the_fly(square: Square, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    let mut current_square;

    for translation in TRANSLATIONS[4..].iter() {
        current_square = square;

        while let Some(sq) = translation(current_square) {
            attacks.set_square(sq);

            if blockers.has_square(sq) {
                break;
            }

            current_square = sq;
        }
    }

    return attacks;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::mask_attacks;
    use crate::{
        bitboard::{attack_tables, BitBoard},
        primitives::{board::Square, File, Rank},
    };

    #[test]
    fn attacks_E4() {
        assert_eq!(
            mask_attacks(Square::E4).to_string(),
            [
                ". . . . . . . .",
                ". x . . . . . .",
                ". . x . . . x .",
                ". . . x . x . .",
                ". . . . . . . .", // 4
                ". . . x . x . .",
                ". . x . . . x .",
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
                ". x . . . . . .",
                ". . x . . . . .",
                ". . . x . . . .",
                ". . . . x . . .",
                ". . . . . x . .",
                ". . . . . . x .",
                ". . . . . . . .",
            ]
            .join("\n")
        );
    }
    #[test]
    fn attacks_A8() {
        assert_eq!(
            mask_attacks(Square::A8).to_string(),
            [
                ". . . . . . . .",
                ". x . . . . . .", // 7
                ". . x . . . . .",
                ". . . x . . . .",
                ". . . . x . . .",
                ". . . . . x . .",
                ". . . . . . x .",
                ". . . . . . . .",
                //a
            ]
            .join("\n")
        );
    }

    #[test]
    fn attacks_on_the_fly_D4() {
        let blockers = BitBoard::new([
            // Squares with pieces
            Square::B6,
            Square::G7,
            Square::E3,
            Square::B2,
        ]);
        assert_eq!(
            blockers.to_string(),
            [
                ". . . . . . . .",
                ". . . . . . x .",
                ". x . . . . . .",
                ". . . . . . . .",
                ". . . . . . . .",
                ". . . . x . . .",
                ". x . . . . . .",
                ". . . . . . . .",
            ]
            .join("\n")
        );

        let attack_map = super::mask_attacks_on_the_fly(Square::D4, blockers);
        assert_eq!(
            attack_map.to_string(),
            [
                ". . . . . . . .",
                ". . . . . . x .",
                ". x . . . x . .",
                ". . x . x . . .",
                ". . . . . . . .",
                ". . x . x . . .",
                ". x . . . . . .",
                ". . . . . . . .",
            ]
            .join("\n")
        );
    }

    #[test]
    fn attack_bits() {
        let string = Rank::ALL
            .iter()
            .map(|rank| {
                File::ALL
                    .iter()
                    .map(|file| {
                        mask_attacks(Square::make_square(*file, *rank))
                            .count_bits()
                            .to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(
            string,
            [
                "6 5 5 5 5 5 5 6",
                "5 5 5 5 5 5 5 5",
                "5 5 7 7 7 7 5 5",
                "5 5 7 9 9 7 5 5",
                "5 5 7 9 9 7 5 5",
                "5 5 7 7 7 7 5 5",
                "5 5 5 5 5 5 5 5",
                "6 5 5 5 5 5 5 6",
            ]
            .join("\n")
        );
    }
}
