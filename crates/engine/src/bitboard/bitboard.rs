use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::core::board::{File, Rank, NUM_FILES};

use crate::core::board::{Square, NUM_RANKS};

#[derive(Debug, Eq, PartialEq, Clone)]
/// A wrapper around a `u64` to represent a bitboard.
///
/// It is worth noting that white side has low values (bits `[0..32]`) and black high values (bits `[32..64]`)
/// Most engines do it the other way around (whoops)
pub struct BitBoard(u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    #[inline]
    /// Creates a bitboard with the given `squares` bites set to 1
    pub fn new<const COUNT: usize>(squares: [Square; COUNT]) -> Self {
        let mut bb = BitBoard::EMPTY;

        for sq in squares {
            bb.set_square(sq);
        }

        return bb;
    }

    #[inline]
    /// Returns the count of the amount of bits are set to 1.
    /// Value: `[0..64]`
    ///
    /// ```
    /// use engine::{bitboard::BitBoard, core::Square};
    ///
    /// assert_eq!(BitBoard::EMPTY.count_bits(), 0);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2]).count_bits(), 2);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2, Square::A3]).count_bits(), 3);
    /// assert_eq!(BitBoard::new([Square::A1, Square::A2, Square::A3, Square::A4, Square::A5]).count_bits(), 5);
    /// ```
    pub fn count_bits(&self) -> u8 {
        let mut amount = 0;
        let mut bits = self.0;

        while bits != 0 {
            bits &= bits - 1;
            amount += 1;
        }

        return amount;
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
    /// Returns `None` if the bitboard is empty
    ///
    /// NOTE: this implementation has low byte values for low ranks (e.g. Rank 1) instead of high
    /// ranks like most chess engines.
    /// 
    /// ```
    /// use engine::{bitboard::BitBoard, core::Square};
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
            .count_bits()
        ));
    }

    #[inline]
    pub fn to_int(&self) -> u64 {
        return self.0;
    }
    #[inline]
    pub fn from_int(bit_value: u64) -> Self {
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

    /// Used to set square bits to `1` on the bitboard
    #[inline]
    pub fn set_square(&mut self, square: Square) -> &mut Self {
        self.bitor_assign(BitBoard::from(square));
        return self;
    }

    /// Used to set square bits to `1` on the bitboard
    #[inline]
    pub fn set_maybe_square(&mut self, square: &Option<Square>) -> &mut Self {
        if let Some(sq) = square {
            self.bitor_assign(BitBoard::from(sq));
        }

        return self;
    }

    /// Used to set square bits to `0` on the bitboard
    #[inline]
    pub fn unset_square(&mut self, square: Square) -> &mut Self {
        if self.has_square(square) {
            self.bitxor_assign(BitBoard::from(square));
        }

        return self;
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        return BitBoard::EMPTY;
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard(1u64 << value.to_int())
    }
}
impl From<&Square> for BitBoard {
    fn from(value: &Square) -> Self {
        BitBoard(1u64 << value.to_int())
    }
}

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

// // Impl Mul
// impl Mul for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }
//
// impl Mul for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }
//
// impl Mul<&BitBoard> for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }
//
// impl Mul<BitBoard> for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }

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

        let mut s = [EMPTY_STRING; NUM_RANKS]
            .iter_mut()
            .enumerate()
            .map(|(ri, row_str)| {
                for fi in 0..NUM_FILES {
                    let file = File::from_index(fi);
                    row_str.push_str(
                        if self.has_square(Square::make_square(file, Rank::from_index(ri))) {
                            "x"
                        } else {
                            "."
                        },
                    );

                    if fi != NUM_FILES - 1 {
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
