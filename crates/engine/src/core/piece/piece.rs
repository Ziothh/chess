use crate::core::{
    board::{Square, NUM_FILES},
    moves::Move,
    piece::{BishopType, KingType, KnightType, PawnType, QueenType, RookType},
    team::Team,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessPieceVariant {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
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
            Bishop => 'B',
            Knight => 'N',
            Rook => 'R',
            Queen => 'Q',
            King => 'K',
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct ChessPiece {
    pub team: Team,
    pub variant: ChessPieceVariant,
}

impl ChessPiece {
    pub fn pseudo_legal_moves(&self, position: Square) -> Vec<Move> {
        use ChessPieceVariant::*;
        match self.variant {
            Pawn => PawnType::pseudo_legal_moves(position, self.team),
            Knight => KnightType::pseudo_legal_moves(position, self.team),
            Bishop => BishopType::pseudo_legal_moves(position, self.team),
            Rook => RookType::pseudo_legal_moves(position, self.team),
            Queen => QueenType::pseudo_legal_moves(position, self.team),
            King => KingType::pseudo_legal_moves(position, self.team),
        }
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

pub trait PieceType {
    const PIECE_VARIANT: ChessPieceVariant;
    //     fn is(piece: Piece) -> bool;
    //     fn into_piece() -> Piece;
    //     #[inline(always)]
    //     fn pseudo_legals(src: Square, color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard;

    /// TODO: find a way to remove &self. This does not need to be an instance
    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move>;

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
}

pub trait SlidingPiece {
    const TRANSLATIONS: &'static [fn(origin: Square) -> Option<Square>];

    fn generate_sliding_destionations(origin: Square) -> Vec<Square> {
        let mut squares = Vec::new();
        for translation in Self::TRANSLATIONS {
            let mut latest = origin;

            for _ in 0..NUM_FILES {
                if let Some(dest) = translation(latest) {
                    squares.push(dest);

                    latest = dest;
                } else {
                  break;
                }
            }
        }

        return squares;
    }
}