use super::board::Rank;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Team {
    Black,
    White,
}

/// How many colors are there?
pub const NUM_TEAMS: usize = 2;
/// List all colors
pub const ALL_TEAMS: [Team; NUM_TEAMS] = [Team::White, Team::Black];

impl Team {
    /// Convert the `Team` to a `usize` for table lookups.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Gets the nth rank, relative to the team starting position. 
    ///
    /// NOTE: `nth` = 1..=8 // ! The rank wraps around if nth > 8
    #[inline]
    pub fn get_nth_rank(&self, nth: usize) -> Rank {
        Rank::from_index(
            (match *self {
                Self::White => nth,
                Self::Black => NUM_TEAMS - nth,
            } - 1),
        )
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
        if self == Team::White {
            Team::Black
        } else {
            Team::White
        }
    }
}
