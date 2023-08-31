use crate::{
    bitboard::BitBoard,
    primitives::{File, Rank, Square},
};

use super::prelude::TRANSLATIONS;

#[rustfmt::skip]
/// The max amount of bits set to 1 in the attack mask (by mask_attack()) for every square
const RELEVANT_BITS: [u32; BitBoard::SIZE] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[rustfmt::skip]
pub fn mask_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    let file = square.get_file();
    let rank = square.get_rank();
    let file_index = file.to_index();
    let rank_index = rank.to_index();

    for r in (rank_index + 1)..=6 { attacks.set_square(Square::make_square(file, Rank::from_index(r))); }
    for r in 1..=(rank_index.checked_sub(1).unwrap_or(0)) { attacks.set_square(Square::make_square(file, Rank::from_index(r))); }
    for f in (file_index + 1)..=6 { attacks.set_square(Square::make_square(File::from_index(f), rank)); }
    for f in 1..=(file_index.checked_sub(1).unwrap_or(0)) { attacks.set_square(Square::make_square(File::from_index(f), rank)); }

    // for translation in TRANSLATIONS[..4].iter() {
    //     let mut current_square = square;
    //
    //     while let Some(sq) = translation(current_square) {
    //         attacks.set_square(sq);
    //         current_square = sq;
    //     }
    // }

    return attacks;
}

pub fn mask_attacks_on_the_fly(square: Square, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    let mut current_square;

    for translation in TRANSLATIONS[..4].iter() {
        current_square = square;

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
    use crate::{bitboard::BitBoard, primitives::board::Square};

    #[test]
    fn attacks_E4() {
        assert_eq!(
            mask_attacks(Square::E4).to_string(),
            [
                ". . . . . . . .",
                ". . . . x . . .",
                ". . . . x . . .",
                ". . . . x . . .",
                ". x x x . x x .", // 4
                ". . . . x . . .",
                ". . . . x . . .",
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
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". . . . . . . x",
                ". x x x x x x .",
            ]
            .join("\n")
        );
    }
    #[test]
    fn attacks_A8() {
        // println!("{}", mask_attacks(Square::A8).to_string(),);
        assert_eq!(
            mask_attacks(Square::A8).to_string(),
            [
                ". x x x x x x .", // 8
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
                "x . . . . . . .",
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
