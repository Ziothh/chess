// Include the generated lookup tables
// include!(concat!(env!("OUT_DIR"), "/magic_gen.rs"));
use crate::{
    bitboard::BitBoard,
    primitives::{File, Rank},
};

include!("/home/zioth/projects/apps/chess/crates/engine/data/magic_gen.rs");

pub mod rook {
    use super::*;
    use crate::{
        bitboard::BitBoard,
        primitives::{SlidingDirection, Square},
    };

    /// Get the rays for a rook on a particular square.
    #[inline]
    pub fn get_rays(sq: Square) -> BitBoard {
        unsafe {
            *RAYS
                .get_unchecked(SlidingDirection::Orthogonal.to_index())
                .get_unchecked(sq.to_index())
        }
    }

    /// Get the moves for a rook on a particular square, given blockers blocking my movement.
    #[inline]
    pub fn get_moves(square: Square, blockers: BitBoard) -> BitBoard {
        unsafe {
            let magic: Magic = *MAGIC_NUMBERS
                .get_unchecked(SlidingDirection::Orthogonal.to_index())
                .get_unchecked(square.to_int() as usize);

            return *MOVES.get_unchecked(
                (magic.offset as usize)
                    + (magic.magic_number * (blockers & magic.mask)).to_size(magic.rightshift),
            ) & get_rays(square);
        }
    }
}

pub mod bishop {
    use super::*;
    use crate::{bitboard::BitBoard, primitives::{Square, SlidingDirection}};

    /// Get the rays for a bishop on a particular square.
    #[inline]
    pub fn get_rays(square: Square) -> BitBoard {
        unsafe { *RAYS.get_unchecked(SlidingDirection::Diagonal.to_index()).get_unchecked(square.to_index()) }
    }

    /// Get the moves for a bishop on a particular square, given blockers blocking my movement.
    #[inline]
    pub fn get_moves(square: Square, blockers: BitBoard) -> BitBoard {
        unsafe {
            let magic: Magic = *MAGIC_NUMBERS
                .get_unchecked(SlidingDirection::Diagonal.to_index())
                .get_unchecked(square.to_int() as usize);
            *MOVES.get_unchecked(
                (magic.offset as usize)
                    + (magic.magic_number * (blockers & magic.mask)).to_size(magic.rightshift),
            ) & get_rays(square)
        }
    }
}

pub mod king {
    use super::*;
    use crate::{bitboard::BitBoard, primitives::Square};

    /// Get the king moves for a particular square.
    #[inline]
    pub fn get_moves(square: Square) -> BitBoard {
        unsafe { *KING_MOVES.get_unchecked(square.to_index()) }
    }
}

pub mod knight {
    use super::*;
    use crate::{bitboard::BitBoard, primitives::Square};

    /// Get the knight moves for a particular square.
    #[inline]
    pub fn get_moves(square: Square) -> BitBoard {
        unsafe { *KNIGHT_MOVES.get_unchecked(square.to_index()) }
    }
}

pub mod pawn {
    use super::*;
    use crate::{bitboard::BitBoard, primitives::{Square, Team}};

    /// Get the pawn capture move for a particular square, given the pawn's color and the potential
    /// victims
    #[inline]
    pub fn get_attacks(square: Square, team: Team, blockers: BitBoard) -> BitBoard {
        unsafe {
            *PAWN_ATTACKS
                .get_unchecked(team.to_index())
                .get_unchecked(square.to_index())
                & blockers
        }
    }

    /// Get the quiet pawn moves (non-captures) for a particular square, given the pawn's color and
    /// the potential blocking pieces.
    #[inline]
    pub fn get_quiets(square: Square, team: Team, blockers: BitBoard) -> BitBoard {
        unsafe {
            if (BitBoard::from_square(square.uforward(team)) & blockers) != BitBoard::EMPTY {
                BitBoard::EMPTY
            } else {
                *PAWN_MOVES
                    .get_unchecked(team.to_index())
                    .get_unchecked(square.to_index())
                    & !blockers
            }
        }
    }

    /// Get all the pawn moves for a particular square, given the pawn's color and the potential
    /// blocking pieces and victims.
    #[inline]
    pub fn get_moves(square: Square, team: Team, blockers: BitBoard) -> BitBoard {
        get_attacks(square, team, blockers) ^ get_quiets(square, team, blockers)
    }

    pub mod double_moves {
        use super::super::*;
        use crate::bitboard::BitBoard;


        #[inline]
        pub fn get_origins() -> BitBoard {
            PAWN_ORIGIN_DOUBLE_MOVES
        }

        #[inline]
        pub fn get_destinations() -> BitBoard {
            PAWN_DEST_DOUBLE_MOVES
        }
    }
}

pub mod rays {
    use super::*;
    use crate::{bitboard::BitBoard, primitives::Square};

    /// Get a line (extending to infinity, which in chess is 8 squares), given two squares.
    /// This line does extend past the squares.
    #[inline]
    pub fn line(sq1: Square, sq2: Square) -> BitBoard {
        unsafe {
            *LINES
                .get_unchecked(sq1.to_index())
                .get_unchecked(sq2.to_index())
        }
    }

    /// Get a line between these two squares, not including the squares themselves.
    #[inline]
    pub fn between(sq1: Square, sq2: Square) -> BitBoard {
        unsafe {
            *BETWEEN
                .get_unchecked(sq1.to_index())
                .get_unchecked(sq2.to_index())
        }
    }
}
