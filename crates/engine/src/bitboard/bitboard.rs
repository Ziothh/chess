use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Mul};

use crate::primitives::{board::{File, Rank, Square}, ChessBoard};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
/// A wrapper around a `u64` to represent a bitboard.
///
/// It is worth noting that white side has low values (bits `[0..32]`) and black high values (bits `[32..64]`)
/// Most engines do it the other way around (whoops)
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline]
    /// Creates a bitboard with the given `squares` bites set to 1
    pub const fn new<const COUNT: usize>(squares: [Square; COUNT]) -> Self {
        let mut bb = BitBoard::EMPTY;

        let mut i = COUNT;

        while i != 0 {
            bb.0 |= 1u64 << squares[i - 1].to_int();
            i-= 1 
        }

        return bb;
    }

    #[inline]
    /// Returns the count of the amount of bits are set to 1.
    /// Value: `[0..64]`
    ///
    /// ```
    /// use engine::{bitboard::BitBoard, primitives::Square};
    ///
    /// assert_eq!(BitBoard::EMPTY.count_bits(), 0);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2]).count_bits(), 2);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2, Square::A3]).count_bits(), 3);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2, Square::A3, Square::A4, Square::A5]).count_bits(), 5);
    /// ```
    pub fn count_bits(&self) -> u32 {
        return self.0.count_ones() as u32;
    }

    #[inline]
    /// Returns `true` if all the bits are set to `0`
    ///
    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(BitBoard::EMPTY.is_empty(), true);
    /// assert_eq!(BitBoard::from_int(1).is_empty(), false);
    /// assert_eq!(BitBoard::from_int(69).is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.to_int() == 0;
    }

    /// Gets the index of the least significant first bit (LS1B)
    ///
    /// Returns `None` if the `BitBoard` is empty
    ///
    /// NOTE: this implementation has low byte values for low ranks (e.g. Rank 1) instead of high
    /// ranks like most chess engines.
    /// 
    /// ```
    /// use engine::{bitboard::BitBoard, primitives::Square};
    ///
    /// assert_eq!(BitBoard::from_int(578712835584952320u64).ls1b_square(), Some(Square::new(11)));
    /// assert_eq!(BitBoard::from_int(1).ls1b_square(), Some(Square::A1));
    /// assert_eq!(BitBoard::EMPTY.ls1b_square(), None);
    /// assert_eq!(BitBoard::new([Square::A2, Square::C5]).ls1b_square(), Some(Square::A2));
    /// assert_eq!(
    ///     BitBoard::new([Square::B1, Square::A2, Square::C5]).ls1b_square(),
    ///     Some(Square::B1)
    /// );
    /// ```
    #[inline]
    pub fn ls1b_square(&self) -> Option<Square> {
        if self.is_empty() {
            return None;
        }

        // bit hack
        return Some(Square::new(
                BitBoard::from_int(
                (
                    // This creates a u64 with only the LS1B remaining
                    self.to_int()
                        & (
                            // Equal to -bb.to_int() in C
                            u64::MAX - self.to_int() + 1
                        )
                ) 
                // Doing "- 1" removes the one remaining bit and adds n amount of 1's to the bits before
                // E.g.: 0x10000000 - 1 = 0x01111111
                - 1,
            )
            // We then count the amount of 1's to get the index
            .count_bits() as u8
        ));
    }

    #[inline]
    pub const fn to_int(&self) -> u64 {
        return self.0;
    }
    #[inline]
    pub const fn from_int(bit_value: u64) -> Self {
        return Self(bit_value);
    }

    /// If the square is not set on this bitboard, it returns `BitBoard(0)`
    #[inline]
    pub fn get_square_bitboard(&self, square: Square) -> BitBoard {
        self & BitBoard::from(square)
    }
    #[inline]
    pub fn has_square(&self, square: Square) -> bool {
        self.get_square_bitboard(square) != BitBoard::EMPTY
    }

    /// Used to set square bit to `1` on the bitboard
    #[inline]
    pub fn set_square(&mut self, square: Square) -> &mut Self {
        self.bitor_assign(BitBoard::from(square));
        return self;
    }

    /// Used to set square bit to `1` on the bitboard
    ///
    /// If the given `square` is equal to `Option::None`, then it leaves the `BitBoard` unchanged.
    #[inline]
    pub fn set_maybe_square(&mut self, square: Option<Square>) -> &mut Self {
        if let Some(sq) = square {
            self.set_square(sq);
        }

        return self;
    }

    /// Used to set square bit to `0` on the bitboard
    #[inline]
    pub fn unset_square(&mut self, square: Square) -> &mut Self {
        if self.has_square(square) {
            self.bitxor_assign(BitBoard::from(square));
        }

        return self;
    }

    /// Used to set square bit to `0` on the bitboard
    ///
    /// If the given `square` is equal to `Option::None`, then it leaves the `BitBoard` unchanged.
    #[inline]
    pub fn unset_maybe_square(&mut self, square: Option<Square>) -> &mut Self {
        if let Some(sq) = square {
            self.unset_square(sq);
        }
        return self;
    }


    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn from_square(square: Square) -> BitBoard {
        BitBoard(1u64 << square.to_int())
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    #[inline]
    pub fn from_maybe_square(square: Option<Square>) -> Option<BitBoard> {
        square.map(|s| BitBoard::from_square(s))
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    #[inline]
    pub fn to_square(&self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    #[inline]
    pub fn reverse_colors(&self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    #[inline]
    pub fn to_size(&self, rightshift: u8) -> usize {
        (self.0 >> rightshift) as usize
    }


    // [constants]

    // [constants:base]
    /// The amount of bits a `BitBoard` constains
    pub const SIZE: usize = ChessBoard::SIZE;

    /// A `BitBoard` where every square bit is set to `0`
    ///
    /// The value of the `BitBoard` is equal to `0`
    pub const EMPTY: BitBoard = BitBoard(u64::MIN);
    /// A `BitBoard` where every square bit is set to `1`
    ///
    /// The value of the `BitBoard` is equal to `0xFFFFFFFFFFFFFFFF`
    pub const FULL: BitBoard = BitBoard(u64::MAX);


    // [constants:magic]
    #[rustfmt::skip]
    /// A `BitBoard` where all bits, except for the A file, are set to 1
    ///
    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::NOT_A_FILE.to_string(),
    ///     [
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///         ". x x x x x x x",
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    pub const NOT_A_FILE: BitBoard = BitBoard(!BitBoard::new([
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
    ///
    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::NOT_AB_FILE.to_string(),
    ///     [
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///         ". . x x x x x x",
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    pub const NOT_AB_FILE: BitBoard = BitBoard(BitBoard::NOT_A_FILE.0 & !BitBoard::new([
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
    ///
    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::NOT_H_FILE.to_string(),
    ///     [
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///         "x x x x x x x .",
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    pub const NOT_H_FILE: BitBoard = BitBoard(!BitBoard::new([
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
    /// 
    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::NOT_GH_FILE.to_string(),
    ///     [
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///         "x x x x x x . .",
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    pub const NOT_GH_FILE: BitBoard = BitBoard(BitBoard::NOT_H_FILE.0 & !BitBoard::new([
        Square::G1,
        Square::G2,
        Square::G3,
        Square::G4,
        Square::G5,
        Square::G6,
        Square::G7,
        Square::G8,
    ]).0);

    /// ```
    /// use engine::bitboard::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::NOT_BORDERS.to_string(),
    ///     [
    ///         ". . . . . . . .",
    ///         ". x x x x x x .",
    ///         ". x x x x x x .",
    ///         ". x x x x x x .",
    ///         ". x x x x x x .",
    ///         ". x x x x x x .",
    ///         ". x x x x x x .",
    ///         ". . . . . . . .",
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    pub const NOT_BORDERS: BitBoard = BitBoard(
        BitBoard::NOT_A_FILE.0
        & BitBoard::NOT_H_FILE.0
        & !BitBoard::new([
            Square::B1,
            Square::C1,
            Square::D1,
            Square::E1,
            Square::F1,
            Square::G1,


            Square::B8,
            Square::C8,
            Square::D8,
            Square::E8,
            Square::F8,
            Square::G8,
        ]).0
    );


    // Given a file, what squares are on that file?
    pub const FILES: [BitBoard; 8] = [
        BitBoard(72340172838076673),
        BitBoard(144680345676153346),
        BitBoard(289360691352306692),
        BitBoard(578721382704613384),
        BitBoard(1157442765409226768),
        BitBoard(2314885530818453536),
        BitBoard(4629771061636907072),
        BitBoard(9259542123273814144),
    ];
    // Given a file, what squares are adjacent to that file?  Useful for detecting passed pawns.
    pub const ADJACENT_FILES: [BitBoard; 8] = [
        BitBoard(144680345676153346),
        BitBoard(361700864190383365),
        BitBoard(723401728380766730),
        BitBoard(1446803456761533460),
        BitBoard(2893606913523066920),
        BitBoard(5787213827046133840),
        BitBoard(11574427654092267680),
        BitBoard(4629771061636907072),
    ];
    /// Given a rank, what squares are on that rank?
    pub const RANKS: [BitBoard; 8] = [
        BitBoard(255),
        BitBoard(65280),
        BitBoard(16711680),
        BitBoard(4278190080),
        BitBoard(1095216660480),
        BitBoard(280375465082880),
        BitBoard(71776119061217280),
        BitBoard(18374686479671623680),
    ];
}

impl Default for BitBoard {
    fn default() -> Self {
        return BitBoard::EMPTY;
    }
}

// [From and Into]
impl From<Square> for BitBoard {
    #[inline]
    fn from(value: Square) -> Self {
        Self::from_square(value)
    }
}
impl From<&Square> for BitBoard {
    #[inline]
    fn from(value: &Square) -> Self {
        Self::from_square(*value)
    }
}

// [Bitwise operations]
// Impl BitAnd
impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}
impl Mul for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}
impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}
impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        return &self.to_int() == other;
    }
}
impl PartialEq<u64> for &BitBoard {
    fn eq(&self, other: &u64) -> bool {
        return &self.to_int() == other;
    }
}

impl std::fmt::Display for BitBoard {
    // #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const EMPTY_STRING: String = String::new();

        let mut s = [EMPTY_STRING; Rank::SIZE]
            .iter_mut()
            .enumerate()
            .map(|(ri, row_str)| {
                for fi in 0..File::SIZE {
                    let file = File::from_index(fi);
                    row_str.push_str(
                        if self.has_square(Square::make_square(file, Rank::from_index(ri))) {
                            "x"
                        } else {
                            "."
                        },
                    );

                    if fi != File::SIZE - 1 {
                        row_str.push_str(" ");
                    }
                }

                return row_str.to_owned();
            })
            .collect::<Vec<_>>();

        s.reverse();

        write!(f, "{}", s.join("\n"))
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
