use crate::{
    bitboard::BitBoard,
    core::{board::Square, ChessBoard, Team},
};

pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
/** Translations that move a square by 1.
 * TRANSLATIONS[..4] are 1D
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
pub type AttackMap = [BitBoard; ChessBoard::SIZE];
/// An array for every team that contains an `AttackMap`
/// @see `AttackMap` for more info
pub type AttackTable = [AttackMap; Team::SIZE];

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

#[rustfmt::skip]
/// A `BitBoard` where all bits, except for the A file, are set to 1
pub const NOT_A_FILE: BitBoard = BitBoard(BitBoard::FULL.0 & !BitBoard::new([
    Square::A1,
    Square::A2,
    Square::A3,
    Square::A4,
    Square::A5,
    Square::A6,
    Square::A7,
    Square::A8,
]).0);

#[rustfmt::skip]
/// A `BitBoard` where all bits, except for the AB files, are set to 1
pub const NOT_AB_FILE: BitBoard = BitBoard(NOT_A_FILE.0 & !BitBoard::new([
    Square::B1,
    Square::B2,
    Square::B3,
    Square::B4,
    Square::B5,
    Square::B6,
    Square::B7,
    Square::B8,
]).0);

#[rustfmt::skip]
/// A `BitBoard` where all bits, except for the H files, are set to 1
pub const NOT_H_FILE: BitBoard = BitBoard(BitBoard::FULL.0 & BitBoard::new([
    Square::H1,
    Square::H2,
    Square::H3,
    Square::H4,
    Square::H5,
    Square::H6,
    Square::H7,
    Square::H8,
]).0);

#[rustfmt::skip]
/// A `BitBoard` where all bits, except for the GH files, are set to 1
pub const NOT_GH_FILE: BitBoard = BitBoard(NOT_H_FILE.0 & !BitBoard::new([
    Square::G1,
    Square::G2,
    Square::G3,
    Square::G4,
    Square::G5,
    Square::G6,
    Square::G7,
    Square::G8,
]).0);
