use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::core::board::{File, Rank, NUM_FILES};

use crate::core::board::{Square, NUM_RANKS};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

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

    #[inline]
    /// Creates a bitboard with the given `squares` bites set to 1
    pub fn new<const COUNT: usize>(squares: [Square; COUNT]) -> Self {
        let mut bb = BitBoard::EMPTY;

        for sq in squares {
            bb.set_square(sq);
        }

        return bb;
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
