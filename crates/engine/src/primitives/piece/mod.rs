use crate::primitives::team::Team;

#[derive(Debug, PartialEq, Clone, Copy, rspc::Type, serde::Serialize, serde::Deserialize)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    /// The number of variants of the enum.
    ///
    /// It is equal to `Self::ALL.len()`.
    pub const SIZE: usize = 6;
    /// An array containing all the variants of the enum
    /// ```rust
    /// // Example
    /// use engine::primitives::Piece;
    ///
    /// let piece = Piece::Pawn;
    /// assert_eq!(piece, Piece::ALL[piece.to_index()]);
    /// ```
    pub const ALL: [Piece; Piece::SIZE] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];

    /// Possible `Piece` variants a pawn can promote to
    pub const PROMOTION_TARGETS: [Self; 4] = [Self::Queen, Self::Knight, Self::Rook, Self::Bishop];

    pub fn is_sliding(&self) -> bool {
        use super::Piece::*;

        match *self {
            Bishop | Queen | Rook => true,
            _ => false,
        }
    }

    /// Gets the variant index in the enum (= nth variant)
    /// ```
    /// // Example
    /// use engine::primitives::Piece;
    ///
    /// assert_eq!(Piece::Pawn.to_index(), 0);
    /// assert_eq!(Piece::Knight.to_index(), 1);
    /// assert_eq!(Piece::Bishop.to_index(), 2);
    /// assert_eq!(Piece::Rook.to_index(), 3);
    /// assert_eq!(Piece::Queen.to_index(), 4);
    /// assert_eq!(Piece::King.to_index(), 5);
    /// ```
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use self::Piece::*;

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

impl ToString for Piece {
    fn to_string(&self) -> String {
        use Piece::*;

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

#[derive(Clone, Copy)]
/// Represent the variants of sliding directions for sliding pieces
pub enum SlidingDirection {
    /// For horizontally and vertically sliding pieces.
    /// Means "intersecting or lying at right angles".
    ///
    /// Example piece: `Piece::Rook`
    Orthogonal,
    /// For diagonally sliding pieces.
    ///
    /// Example piece: `Piece::Bishop`
    Diagonal,
}
impl SlidingDirection {
    pub const SIZE: usize = 2;
    pub const ALL: [Self; Self::SIZE] = [Self::Orthogonal, Self::Diagonal];

    pub fn to_index(&self) -> usize {
        return *self as usize;
    }
}

#[derive(Debug, PartialEq, Clone, Copy, rspc::Type, serde::Serialize, serde::Deserialize)]
pub struct ChessPiece {
    pub team: Team,
    pub variant: Piece,
}

/// The unice value of the first chess piece character, the white king
///
/// The chess piece characters range from `[0x2654, 0x265F]`.
///
/// Black starts at `0x265A`.
const CHESS_PIECE_UNICODE_START: u32 = '♔' as u32;

impl ChessPiece {
    /// Returns the unicode character for a given `ChessPiece`
    ///
    /// NOTE: the white pieces are outlined (♙) and the black ones are filled (♟).
    /// If you have a dark background and a light font,
    /// the characters look like they're on the wrong team.
    #[rustfmt::skip]
    pub fn to_unicode(&self) -> char {
        char::from_u32(
            CHESS_PIECE_UNICODE_START
            + (((Piece::SIZE - 1) - self.variant.to_index())
                + match self.team {
                    Team::White => 0,
                    Team::Black => Piece::SIZE,
                }) as u32,
        )
        .expect("The given unicode value to be a valid")
    }

    // pub fn is_sliding(&self) -> bool {
    //   use self::Piece::*;
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
            variant: Piece::try_from(value)?,
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
            .cartesian_product(Piece::ALL)
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
