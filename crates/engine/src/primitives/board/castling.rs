use crate::{
    bitboard::BitBoard,
    primitives::{File, Square, Team},
    utils::enums::ArrayEnum,
};

/// What castle rights does a particular player have?
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum CastleRights {
    NoRights = 0,
    KingSide = 1,
    QueenSide = 2,
    Both = 3,
}

impl CastleRights {
    pub const SIZE: usize = 4;
    pub const ALL: [CastleRights; Self::SIZE] = [
        CastleRights::NoRights,
        CastleRights::KingSide,
        CastleRights::QueenSide,
        CastleRights::Both,
    ];

    pub const PER_SQUARE: [[u8; Square::AMOUNT]; Team::SIZE] = [
        [
            2, 0, 0, 0, 3, 0, 0, 1, // 1
            0, 0, 0, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 0, 0, // 5
            0, 0, 0, 0, 0, 0, 0, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, // 7
            0, 0, 0, 0, 0, 0, 0, 0, // 8
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1
            0, 0, 0, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 0, 0, // 5
            0, 0, 0, 0, 0, 0, 0, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, // 7
            2, 0, 0, 0, 3, 0, 0, 1, // 8
        ],
    ];

    pub const KINGSIDE_CASTLE_SQUARES: [BitBoard; Team::SIZE] = {
        let white_back_rank = Team::White.get_nth_rank(1);
        let black_back_rank = Team::Black.get_nth_rank(1);

        [
            BitBoard::new([
                Square::make_square(File::F, white_back_rank),
                Square::make_square(File::G, white_back_rank),
            ]),
            BitBoard::new([
                Square::make_square(File::F, black_back_rank),
                Square::make_square(File::G, black_back_rank),
            ]),
        ]
    };

    pub const QUEENSIDE_CASTLE_SQUARES: [BitBoard; Team::SIZE] = {
        let white_back_rank = Team::White.get_nth_rank(1);
        let black_back_rank = Team::Black.get_nth_rank(1);

        [
            BitBoard::new([
                Square::make_square(File::B, white_back_rank),
                Square::make_square(File::C, white_back_rank),
                Square::make_square(File::D, white_back_rank),
            ]),
            BitBoard::new([
                Square::make_square(File::B, black_back_rank),
                Square::make_square(File::C, black_back_rank),
                Square::make_square(File::D, black_back_rank),
            ]),
        ]
    };

    pub const MOVES: BitBoard = BitBoard::new([
        Square::C1,
        Square::C8,
        Square::E1,
        Square::E8,
        Square::G1,
        Square::G8,
    ]);

    /// Can I castle kingside?
    pub fn has_kingside(&self) -> bool {
        self.to_index() & 1 == 1
    }

    /// Can I castle queenside?
    pub fn has_queenside(&self) -> bool {
        self.to_index() & 2 == 2
    }

    pub fn square_to_castle_rights(team: Team, sq: Square) -> CastleRights {
        CastleRights::from_index(unsafe {
            *Self::PER_SQUARE
                .get_unchecked(team.to_index())
                .get_unchecked(sq.to_index())
        } as usize)
    }

    /// What squares need to be empty to castle kingside?
    pub fn kingside_squares(&self, team: Team) -> BitBoard {
        unsafe { *Self::KINGSIDE_CASTLE_SQUARES.get_unchecked(team.to_index()) }
    }

    /// What squares need to be empty to castle queenside?
    pub fn queenside_squares(&self, team: Team) -> BitBoard {
        unsafe { *Self::QUEENSIDE_CASTLE_SQUARES.get_unchecked(team.to_index()) }
    }

    /// Remove castle rights, and return a new `CastleRights`.
    pub fn remove(&self, value: CastleRights) -> CastleRights {
        CastleRights::from_index(self.to_index() & !value.to_index())
    }

    /// Add some castle rights, and return a new `CastleRights`.
    pub fn add(&self, value: CastleRights) -> CastleRights {
        CastleRights::from_index(self.to_index() | value.to_index())
    }

    /// Convert `CastleRights` to `usize` for table lookups
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Convert `usize` to `CastleRights`.
    pub fn from_index(i: usize) -> CastleRights {
        match i & 3 {
            0 => CastleRights::NoRights,
            1 => CastleRights::KingSide,
            2 => CastleRights::QueenSide,
            3 => CastleRights::Both,
            _ => unreachable!(),
        }
    }

    /// Which rooks can we "guarantee" we haven't moved yet?
    pub fn unmoved_rooks(&self, team: Team) -> BitBoard {
        match *self {
            CastleRights::NoRights => BitBoard::EMPTY,
            CastleRights::KingSide => BitBoard::set(File::H, team.get_backrank()),
            CastleRights::QueenSide => BitBoard::set(File::A, team.get_backrank()),
            CastleRights::Both => {
                BitBoard::set(File::A, team.get_backrank())
                    ^ BitBoard::set(File::H, team.get_backrank())
            }
        }
    }

    /// Convert the castle rights to an FEN compatible string.
    ///
    /// ```
    /// use engine::{CastleRights, Team};
    ///
    /// assert_eq!(CastleRights::NoRights.to_string(Team::White), "");
    /// assert_eq!(CastleRights::Both.to_string(Team::Black), "kq");
    /// assert_eq!(CastleRights::KingSide.to_string(Team::White), "K");
    /// assert_eq!(CastleRights::QueenSide.to_string(Team::Black), "q");
    /// ```
    pub fn to_string(&self, team: Team) -> String {
        let result = match *self {
            CastleRights::NoRights => "",
            CastleRights::KingSide => "k",
            CastleRights::QueenSide => "q",
            CastleRights::Both => "kq",
        };

        match team {
            Team::White => result.to_uppercase(),
            Team::Black => result.to_string(),
        }
    }

    /// Given a square of a rook, which side is it on?
    pub fn rook_square_to_castle_rights(square: Square) -> CastleRights {
        match square.get_file() {
            File::A => CastleRights::QueenSide,
            File::H => CastleRights::KingSide,
            _ => CastleRights::NoRights,
        }
    }
}
