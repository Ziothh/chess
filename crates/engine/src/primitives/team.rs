use crate::utils::enums::ArrayEnum;

use super::board::Rank;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[derive(rspc::Type, serde::Serialize, serde::Deserialize)]
pub enum Team {
    White,
    Black,
}

impl ArrayEnum<2> for Team {
    const ALL: [Self; Self::SIZE] = [Self::White, Self::Black];

    #[inline]
    fn to_index(&self) -> usize {
        *self as usize
    }

}

impl Team {
    pub fn try_from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::White),
            1 => Some(Self::Black),
            _ => None,
        }
    }

    /// Gets the nth rank, relative to the team starting position.
    ///
    /// NOTE: `nth` in range [1, 8] // ! The rank wraps around if nth > 8
    #[inline]
    pub const fn get_nth_rank(&self, nth: usize) -> Rank {
        Rank::from_index(match *self {
            Self::White => nth - 1,
            Self::Black => (Rank::SIZE) - nth,
        })
    }

    /// Convert a `Team` to its backrank, which represents the starting rank for its pieces
    ///
    /// ```
    /// use engine::primitives::Team;
    ///
    /// let team = Team::White;
    /// assert_eq!(team.get_backrank(), team.get_nth_rank(1));
    ///
    /// let team = Team::Black;
    /// assert_eq!(team.get_backrank(), team.get_nth_rank(1));
    /// ```
    #[inline]
    pub fn get_backrank(&self) -> Rank {
        match *self {
            Team::White => Rank::First,
            Team::Black => Rank::Eighth,
        }
    }
}

impl From<char> for Team {
    // ! This is not optimal or safe
    fn from(value: char) -> Self {
        if value.is_uppercase() {
            Self::White
        } else {
            Self::Black
        }
    }
}

impl std::ops::Not for Team {
    type Output = Self;

    /// Get the other color.
    #[inline]
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
