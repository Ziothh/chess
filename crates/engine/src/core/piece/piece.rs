use crate::core::{
    board::{ChessBoard, Square},
    moves::{generate::generate_move_data, Move},
    team::Team,
};

#[derive(Debug, PartialEq, Clone, Copy, rspc::Type, serde::Serialize, serde::Deserialize)]
pub enum ChessPieceVariant {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPieceVariant {
    const SIZE: usize = 6;
    const ALL: [ChessPieceVariant; ChessPieceVariant::SIZE] = [
        ChessPieceVariant::Pawn,
        ChessPieceVariant::Knight,
        ChessPieceVariant::Bishop,
        ChessPieceVariant::Rook,
        ChessPieceVariant::Queen,
        ChessPieceVariant::King,
    ];

    pub fn is_sliding(&self) -> bool {
        use super::ChessPieceVariant::*;

        match *self {
            Bishop | Queen | Rook => true,
            _ => false,
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<char> for ChessPieceVariant {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use self::ChessPieceVariant::*;

        match value.to_ascii_uppercase() {
            'P' => Ok(Pawn),
            'B' => Ok(Bishop),
            'N' => Ok(Knight),
            'R' => Ok(Rook),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            _ => Err("\"{value}\" is not a valid chess piece representation".to_owned()),
        }
    }
}

impl ToString for ChessPieceVariant {
    fn to_string(&self) -> String {
        use ChessPieceVariant::*;

        match self {
            Pawn => 'P',
            Knight => 'N',
            Bishop => 'B',
            Rook => 'R',
            Queen => 'Q',
            King => 'K',
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq, rspc::Type, serde::Serialize, serde::Deserialize)]
pub struct ChessPiece {
    pub team: Team,
    pub variant: ChessPieceVariant,
}

/// The unice value of the first chess piece character, the white king
///
/// The chess piece characters range from `[0x2654, 0x265F]`.
///
/// Black starts at `0x265A`.
const CHESS_PIECE_UNICODE_START: u32 = '♔' as u32;

impl ChessPiece {
    pub fn pseudo_legal_moves(
        &self,
        position: Square,
        board: &ChessBoard,
    ) -> (Vec<Vec<Square>>, Vec<Move>) {
        generate_move_data(self, position, board)
    }

    /// Returns the unicode character for a given `ChessPiece`
    ///
    /// NOTE: the white pieces are outlined (♙) and the black ones are filled (♟).
    /// If you have a dark background and a light font, 
    /// the characters look like they're on the wrong team.
    #[rustfmt::skip]
    pub fn to_unicode(&self) -> char {
        char::from_u32(
            CHESS_PIECE_UNICODE_START
            + (((ChessPieceVariant::SIZE - 1) - self.variant.to_index())
                + match self.team {
                    Team::White => 0,
                    Team::Black => ChessPieceVariant::SIZE,
                }) as u32,
        )
        .expect("The given unicode value to be a valid")
    }

    // pub fn is_sliding(&self) -> bool {
    //   use self::ChessPieceVariant::*;
    //
    //   match self.variant {
    //     Bishop => true,
    //     Rook => true,
    //     Queen => true,
    //     _ => false,
    //   }
    // }
}

impl ToString for ChessPiece {
    fn to_string(&self) -> String {
        let mut char = self.variant.to_string();

        if self.team == Team::Black {
            char = char.to_lowercase();
        }

        return char;
    }
}

impl TryFrom<char> for ChessPiece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            team: Team::from(value),
            variant: ChessPieceVariant::try_from(value)?,
        })
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn chess_piece_to_unicode() {
        /// White king value
        const WHITE_START: u32 = 0x2654;
        /// Black king value
        const BLACK_START: u32 = 0x265A;
        /// Black pawn value
        const END: u32 = 0x265F;

        Team::ALL
            .iter()
            .zip([WHITE_START..=(BLACK_START - 1), BLACK_START..=END])
            .cartesian_product(ChessPieceVariant::ALL)
            .for_each(|((team, range), variant)| {
                assert!(range.contains(
                    &(ChessPiece {
                        variant,
                        team: *team
                    }
                    .to_unicode() as u32)
                ))
            });
    }
}

// pub trait PieceType {
//     const PIECE_VARIANT: ChessPieceVariant;
//     //     fn is(piece: Piece) -> bool;
//     //     fn into_piece() -> Piece;
//     //     #[inline(always)]
//     //     fn pseudo_legals(src: Square, color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard;
//
//     /// TODO: find a way to remove &self. This does not need to be an instance
//     fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move>;

//     #[inline(always)]
//     fn legals<T>(movelist: &mut MoveList, board: &Board, mask: BitBoard)
//     where
//         T: CheckType,
//     {
//         let combined = board.combined();
//         let color = board.side_to_move();
//         let my_pieces = board.color_combined(color);
//         let ksq = board.king_square(color);
//
//         let pieces = board.pieces(Self::into_piece()) & my_pieces;
//         let pinned = board.pinned();
//         let checkers = board.checkers();
//
//         let check_mask = if T::IN_CHECK {
//             between(checkers.to_square(), ksq) ^ checkers
//         } else {
//             !EMPTY
//         };
//
//         for src in pieces & !pinned {
//             let moves = Self::pseudo_legals(src, color, *combined, mask) & check_mask;
//             if moves != EMPTY {
//                 unsafe {
//                     movelist.push_unchecked(SquareAndBitBoard::new(src, moves, false));
//                 }
//             }
//         }
//
//         if !T::IN_CHECK {
//             for src in pieces & pinned {
//                 let moves = Self::pseudo_legals(src, color, *combined, mask) & line(src, ksq);
//                 if moves != EMPTY {
//                     unsafe {
//                         movelist.push_unchecked(SquareAndBitBoard::new(src, moves, false));
//                     }
//                 }
//             }
//         }
//     }
// }

// pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
// pub trait PieceMovement {
//     const TRANSLATIONS: &'static [SquareTranslation] = [
//         // Horizontal + vertical
//         Self::N,
//         Self::W,
//         Self::S,
//         Self::E,
//
//         // Diagonal
//         Self::NE,
//         Self::NW,
//         Self::SE,
//         Self::SW,
//     ];
//
//     const IS_SLIDING: bool = true;
//     const MAX_DIRECTION_OFFSET: u8 = if Self::IS_SLIDING {
//         0
//     } else {
//         std::cmp::max(NUM_FILES, NUM_RANKS)
//     };
//
//
//     /// Generates rays for every PieceMovement::TRANSLATIONS direction
//     /// IS ALREADY IMPLEMENTED
//     fn generate_rays(origin: Square, team: Team) -> Vec<Vec<Square>> {
//         Self::TRANSLATIONS
//             .iter()
//             .map(|translate_square| {
//                 (0..Self::MAX_DIRECTION_OFFSET)
//                     .scan(origin, |current_dest, direction_offset| {
//                         // Check if the the square is inside of the board, breaks if not
//                         translate_square(current_dest)
//                     })
//                     .collect()
//             })
//             .collect()
//     }
//
//     // ///
//     // /// IS ALREADY IMPLEMENTED
//     // fn generate_destination_squares(origin: Square, team: Team, board: &ChessBoard) -> Vec<Square> {
//     //     Self::generate_rays(origin, team)
//     //         .iter()
//     //         .map(|direction| {
//     //             direction
//     //                 .iter()
//     //                 .filter(|square| !board.get(square).is_some_and(|piece| piece.team == team))
//     //                 .collect()
//     //         })
//     //         .flatten()
//     //         .collect()
//     // }
// }
