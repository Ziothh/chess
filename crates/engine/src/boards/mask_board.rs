use std::fmt::Display;

use crate::bitboard::BitBoard;
use crate::game::moves::{Move, MoveGen};
use crate::magic;
use crate::primitives::{CastleRights, ChessPiece, File, Piece, Rank, Square, Team};

use itertools::Itertools;
// use serde_big_array::BigArray;

// #[derive(rspc::Type, serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct Board {
    // #[serde(with = "BigArray")] [Option<ChessPiece>; ChessBoard::SIZE]
    /// A `BitBoard` array for every `Piece`.
    /// Every `BitBoard` is a mask of the piece type locations.
    pub piece_masks: [BitBoard; Piece::SIZE],

    /// A `BitBoard` array for every `Team`.
    /// Every `BitBoard` is a mask of the team piece locations.
    pub team_masks: [BitBoard; Team::SIZE],

    pub team_to_move: Team,
    pub castle_rights: [CastleRights; Team::SIZE],
    pub pinned: BitBoard,
    pub checkers: BitBoard,
    hash: u64,
    pub en_passant: Option<Square>,
}

/// What is the status of this game?
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum BoardStatus {
    Ongoing,
    Stalemate,
    Checkmate,
}

impl Board {
    pub const SIZE: usize = File::SIZE * Rank::SIZE;
    // pub const EMPTY_CELL: Option<ChessPiece> = None;

    // [Constructor functions]
    /// Creates a `Board` with the given `cells`
    pub fn new(cells: [Option<ChessPiece>; Board::SIZE]) -> Self {
        let mut board = Self::empty();

        for (index, piece) in cells.into_iter().enumerate() {
            if let Some(ChessPiece { team, variant }) = piece {
                board.set(variant, Square::new(index as u8), team);
            }
        }

        return board;
    }

    /// Creates a `ChessBoard` with all squares being empty
    pub fn empty() -> Self {
        // TODO: make this better
        Self {
            team_masks: [BitBoard::EMPTY; Team::SIZE],
            piece_masks: [BitBoard::EMPTY; Piece::SIZE],
            en_passant: None,
            hash: 0,
            pinned: BitBoard::EMPTY,
            checkers: BitBoard::EMPTY,
            team_to_move: Team::White,
            castle_rights: [CastleRights::NoRights; Team::SIZE],
        }
    }

    // [Instance methods]
    // [[Square methods]]
    pub fn get(&self, square: Square) -> Option<ChessPiece> {
        let mask = BitBoard::from_square(square);

        self.team_masks
            .iter()
            .enumerate()
            .cartesian_product(self.piece_masks.iter().enumerate())
            .find(|((ti, tbb), (pi, pbb))| *tbb & *pbb & mask != BitBoard::EMPTY)
            .map(|((ti, _), (pi, __))| ChessPiece {
                team: Team::try_from_index(ti).unwrap(),
                variant: Piece::ALL[pi],
            })
    }
    pub fn get_by_team(&self, square: Square, team: Team) -> Option<Piece> {
        let team_mask = self.team_mask(team);
        let square_mask = BitBoard::from_square(square);

        for variant in Piece::ALL.into_iter() {
            if self.piece_mask(variant) & square_mask != BitBoard::EMPTY {
                return Some(variant);
            }
        }

        return None;
    }
    pub fn get_mask(&self, piece: Piece, team: Team) -> BitBoard {
        self.piece_mask(piece) & self.team_mask(team)
    }
    pub fn set(&mut self, piece: Piece, square: Square, team: Team) -> &mut Self {
        let bb = BitBoard::from(square);

        self.team_masks[team.to_index()] |= bb;
        self.piece_masks[piece.to_index()] |= bb;

        return self;
    }
    pub fn set_mask(&mut self, piece: Piece, mask: BitBoard, team: Team) -> &mut Self {
        self.team_masks[team.to_index()] |= mask;
        self.piece_masks[piece.to_index()] |= mask;

        return self;
    }
    pub fn remove(&mut self, piece: Piece, square: Square, team: Team) -> &mut Self {
        let bb = !BitBoard::from(square);

        self.team_masks[team.to_index()] &= bb;
        self.piece_masks[piece.to_index()] &= bb;

        return self;
    }
    pub fn remove_mask(&mut self, piece: Piece, mask: BitBoard, team: Team) -> &mut Self {
        self.team_masks[team.to_index()] &= !mask;
        self.piece_masks[piece.to_index()] &= !mask;

        return self;
    }
    pub fn remove_any(&mut self, square: Square) -> &mut Self {
        let bb = !BitBoard::from(square);

        self.team_masks
            .iter_mut()
            .for_each(|team_mask| *team_mask &= bb);
        self.piece_masks
            .iter_mut()
            .for_each(|piece_mask| *piece_mask &= bb);

        return self;
    }
    /// Add or remove a piece from the bitboards in this struct.
    /// It `xor`s the given `bitboard` with the piece, team and combined masks
    fn xor(&mut self, piece: Piece, bitboard: BitBoard, team: Team) {
        unsafe {
            *self.piece_masks.get_unchecked_mut(piece.to_index()) ^= bitboard;
            *self.team_masks.get_unchecked_mut(team.to_index()) ^= bitboard;
            // self.hash ^= Zobrist::piece(piece, bitboard.to_square(), team);
        }
    }
    /// Swaps the values of cell indices `a` and `b`.
    /// `a <-> b`
    ///
    /// TODO: TEST THIS
    pub fn swap(&mut self, a: Square, b: Square) -> &mut Self {
        todo!()
    }

    #[inline]
    pub fn piece_mask(&self, piece: Piece) -> BitBoard {
        self.piece_masks[piece.to_index()]
    }
    #[inline]
    pub fn team_mask(&self, team: Team) -> BitBoard {
        self.team_masks[team.to_index()]
    }
    #[inline]
    pub fn all_mask(&self) -> BitBoard {
        self.team_masks[0] | self.team_masks[1]
    }

    /// What piece is on a particular `Square`?  Is there even one?
    ///
    /// ```
    /// use engine::{ChessBoard, Piece, Square};
    ///
    /// let board = ChessBoard::default();
    ///
    /// assert_eq!(board.piece_on(Square::A1), Some(Piece::Rook));
    /// assert_eq!(board.piece_on(Square::D4), None);
    /// ```
    #[inline]
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        let mask = BitBoard::from_square(square);

        if self.all_mask() & mask == BitBoard::EMPTY {
            return None;
        }

        return Piece::ALL
            .iter()
            .find(|piece| (self.piece_mask(**piece) & mask) != BitBoard::EMPTY)
            .map(|piece| *piece);
    }

    /// What team piece is on a particular square?
    #[inline]
    pub fn team_on(&self, square: Square) -> Option<Team> {
        if (self.team_mask(Team::White) & BitBoard::from_square(square)) != BitBoard::EMPTY {
            Some(Team::White)
        } else if (self.team_mask(Team::Black) & BitBoard::from_square(square)) != BitBoard::EMPTY {
            Some(Team::Black)
        } else {
            None
        }
    }

    /// Give me the `Square` the `color` king is on.
    ///
    /// ```
    /// use chess::{Board, Square, Team};
    ///
    /// let board = Board::default();
    ///
    /// assert_eq!(board.king_square(Team::White), Square::E1);
    /// assert_eq!(board.king_square(Team::Black), Square::E8);
    /// ```
    #[inline]
    pub fn king_square(&self, team: Team) -> Square {
        (self.piece_mask(Piece::King) & self.team_mask(team)).to_square()
    }

    pub fn castle_rights(&self, team: Team) -> CastleRights {
        self.castle_rights[team.to_index()]
    }

    // [Game related methods]

    /// Make a chess move onto an already allocated `Board`.
    ///
    /// panic!() if king is captured.
    ///
    /// ```
    /// use chess::{Board, Move, Square, Team};
    ///
    /// let m = Move::new(Square::D2,
    ///                        Square::D4);
    ///
    /// let mut board = Board::default();
    /// board.make_move(m);
    /// assert_eq!(board.side_to_move(), Team::Black);
    /// ```
    #[inline]
    pub fn make_move(&mut self, m: Move) -> &mut Self {
        // *result = *self;
        self.en_passant = None; // result.remove_ep();

        self.checkers = BitBoard::EMPTY;
        self.pinned = BitBoard::EMPTY;

        let Move {
            origin,
            destination,
            ..
        } = m;

        let origin_bb = BitBoard::from_square(origin);
        let dest_bb = BitBoard::from_square(destination);
        let move_bb = origin_bb ^ dest_bb;
        let moved_piece = self.piece_on(origin).unwrap();

        // Move the piece to the destination
        self.remove(moved_piece, origin, self.team_to_move);
        if let Some(captured) = self.get(destination) {
            self.remove(captured.variant, destination, captured.team);
        }
        self.set(moved_piece, destination, self.team_to_move);

        // Update castling rights
        self.remove_castle_rights(
            !self.team_to_move,
            CastleRights::square_to_castle_rights(!self.team_to_move, destination),
        );
        self.remove_castle_rights(
            self.team_to_move,
            CastleRights::square_to_castle_rights(self.team_to_move, origin),
        );

        let opponent_king = self.get_mask(Piece::King, !self.team_to_move);
        let opponent_king_square = opponent_king.to_square();

        let has_castled = moved_piece == Piece::King && (move_bb & CastleRights::MOVES) == move_bb;

        const CASTLE_ROOK_START: [File; 8] = [
            File::A,
            File::A,
            File::A,
            File::A,
            File::H,
            File::H,
            File::H,
            File::H,
        ];
        const CASTLE_ROOK_END: [File; 8] = [
            File::D,
            File::D,
            File::D,
            File::D,
            File::F,
            File::F,
            File::F,
            File::F,
        ];

        let combined_mask = self.all_mask();

        if moved_piece == Piece::Knight {
            self.checkers ^= magic::knight::get_moves(opponent_king_square) & dest_bb;
        } else if moved_piece == Piece::Pawn {
            if let Some(Piece::Knight) = m.promotion {
                // self.xor(Piece::Pawn, dest_bb, self.side_to_move);
                self.remove_mask(Piece::Pawn, dest_bb, self.team_to_move);
                // self.set
                self.set_mask(Piece::Knight, dest_bb, self.team_to_move);
                self.checkers ^= magic::knight::get_moves(opponent_king_square) & dest_bb;
            } else if let Some(promotion) = m.promotion {
                self.remove_mask(Piece::Pawn, dest_bb, self.team_to_move);
                self.set_mask(promotion, dest_bb, self.team_to_move);
            } else if (origin_bb & magic::pawn::double_moves::get_origins()) != BitBoard::EMPTY
                && (dest_bb & magic::pawn::double_moves::get_destinations()) != BitBoard::EMPTY
            {
                self.en_passant = Some(destination);
                self.checkers ^=
                    magic::pawn::get_attacks(opponent_king_square, !self.team_to_move, dest_bb);
            } else if Some(destination.ubackward(self.team_to_move)) == self.en_passant {
                self.xor(
                    Piece::Pawn,
                    BitBoard::from_square(destination.ubackward(self.team_to_move)),
                    !self.team_to_move,
                );
                self.checkers ^=
                    magic::pawn::get_attacks(opponent_king_square, !self.team_to_move, dest_bb);
            } else {
                self.checkers ^=
                    magic::pawn::get_attacks(opponent_king_square, !self.team_to_move, dest_bb);
            }
        } else if has_castled {
            let my_backrank = self.team_to_move.get_backrank();
            let index = destination.get_file().to_index();
            let start = BitBoard::set(
                unsafe { *CASTLE_ROOK_START.get_unchecked(index) },
                my_backrank,
            );
            let end = BitBoard::set(
                unsafe { *CASTLE_ROOK_END.get_unchecked(index) },
                my_backrank,
            );
            // self.xor(Piece::Rook, start, self.team_to_move);
            // self.xor(Piece::Rook, end, self.team_to_move);
            self.remove_mask(Piece::Rook, start, self.team_to_move);
            self.set_mask(Piece::Rook, end, self.team_to_move);
        }
        // now, lets see if we're in check or pinned
        let attackers = self.team_mask(self.team_to_move)
            & ((magic::bishop::get_rays(opponent_king_square)
                & (self.piece_mask(Piece::Bishop) | self.piece_mask(Piece::Queen)))
                | (magic::rook::get_rays(opponent_king_square)
                    & (self.piece_mask(Piece::Rook) | self.piece_mask(Piece::Queen))));

        for square in attackers {
            let between = magic::rays::between(square, opponent_king_square) & combined_mask;
            if between == BitBoard::EMPTY {
                self.checkers ^= BitBoard::from_square(square);
            } else if between.count_bits() == 1 {
                self.pinned ^= between;
            }
        }

        self.team_to_move = !self.team_to_move;

        return self;
    }

    // [Util functions]

    /// Remove castle rights for a particular side.
    #[inline]
    fn remove_castle_rights(&mut self, team: Team, rights: CastleRights) {
        self.castle_rights[team.to_index()] = self.castle_rights(team).remove(rights);
    }

    #[inline]
    pub fn iter_moves(&self) -> MoveGen {
        MoveGen::new_legal(self)
    }

    // Returns a `Vec<Move>` of all legal moves
    pub fn gen_moves(&self) -> Vec<Move> {
        MoveGen::new_legal(self).collect()
    }


    /// Is this game Ongoing, is it Stalemate, or is it Checkmate?
    ///
    /// ```
    /// use chess::{Board, BoardStatus, Square, ChessMove};
    ///
    /// let mut board = Board::default();
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board = board.make_move_new(ChessMove::new(Square::E2,
    ///                                            Square::E4,
    ///                                            None));
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board = board.make_move_new(ChessMove::new(Square::F7,
    ///                                            Square::F6,
    ///                                            None));
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board = board.make_move_new(ChessMove::new(Square::D2,
    ///                                            Square::D4,
    ///                                            None));
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board = board.make_move_new(ChessMove::new(Square::G7,
    ///                                            Square::G5,
    ///                                            None));
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board = board.make_move_new(ChessMove::new(Square::D1,
    ///                                            Square::H5,
    ///                                            None));
    ///
    /// assert_eq!(board.status(), BoardStatus::Checkmate);
    /// ```
    #[inline]
    pub fn status(&self) -> BoardStatus {
        let moves = self.iter_moves().len();
        match moves {
            0 => {
                if self.checkers.is_empty() {
                    BoardStatus::Stalemate
                } else {
                    BoardStatus::Checkmate
                }
            }
            _ => BoardStatus::Ongoing,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n   {:?} to move\n\n", self.team_to_move)?;

        for rank in Rank::ALL.iter().rev() {
            write!(f, "{rank} ")?;

            for file in File::ALL.iter() {
                write!(f, " ")?;

                let square = Square::make_square(*file, *rank);
                let bb = BitBoard::from(square);

                let Some(variant_index) = self.piece_masks.iter().find_position(|mask| *mask & bb != BitBoard::EMPTY).map(|(index, _)| index) else {
                    write!(f, ".")?;
                    continue;
                };
                let Some(team_index) = self.team_masks.iter().find_position(|mask| *mask & bb != BitBoard::EMPTY).map(|(index, _)| index) else {
                    write!(f, ".")?;
                    continue;
                };

                write!(
                    f,
                    "{}",
                    ChessPiece {
                        variant: Piece::ALL[variant_index],
                        team: Team::ALL[team_index],
                    }
                    .to_string()
                )?;
            }

            write!(f, "\n")?;
        }

        write!(
            f,
            "\n   {}",
            File::ALL.iter().map(|file| file.to_string()).join(" ")
        )?;

        return Ok(());
    }
}
