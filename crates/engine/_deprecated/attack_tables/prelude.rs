use std::fs::File;

use crate::{
    bitboard::BitBoard,
    primitives::{board::Square, ChessBoard},
};

pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
/** Translations that move a square by 1.
 * TRANSLATIONS[..4] are 1D (horizontal & vertical)
 * TRANSLATIONS[4..] are 2D (diagonal) */
pub const TRANSLATIONS: [SquareTranslation; 8] = [
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

/// An array of size 64, that contains an attack `BitBoard` for every square (= the index of the array)
// pub type AttackTable = [BitBoard; ChessBoard::SIZE];
pub type AttackMap = [BitBoard; ChessBoard::SIZE];
// /// An array for every team that contains an `AttackMap`
// /// @see `AttackMap` for more info

pub fn generate_attack_map(bitboard_generator: impl Fn(Square) -> BitBoard) -> AttackMap {
    let mut attack_map: AttackMap = [BitBoard::EMPTY; ChessBoard::SIZE];

    attack_map
        .iter_mut()
        .enumerate()
        .for_each(|(index, bitboard)| {
            *bitboard = bitboard_generator(Square::new(index as u8));
        });

    return attack_map;
}

pub struct AttackTable {
    name: String,
    values: [BitBoard; ChessBoard::SIZE],
}
impl AttackTable {
    pub fn new(name: &str, attack_map_generator: impl Fn(Square) -> BitBoard) -> Self {
        let mut attacks: [BitBoard; ChessBoard::SIZE] = [BitBoard::EMPTY; ChessBoard::SIZE];

        attacks.iter_mut().enumerate().for_each(move |(index, bb)| {
            *bb = attack_map_generator(Square::new(index as u8));
        });

        Self {
            name: name.to_owned(),
            values: attacks,
        }
    }

    pub fn to_array(&self) -> [BitBoard; ChessBoard::SIZE] {
        self.values
    }

    // #[allow(unused)]
    pub fn write_to_out_dir(file: &mut File) -> ! {
        todo!()
    }
}
