use crate::{boards::Board, bitboard::BitBoard, primitives::Piece, magic};

use super::{piece_moves::{BBMove, PawnType, KnightType, BishopType, RookType, QueenType, KingType, PieceMovegen}, Move};

/// An incremental move generator
///
/// This structure enumerates moves slightly slower than board.enumerate_moves(...),
/// but has some extra features, such as:
///
/// * Being an iterator
/// * Not requiring you to create a buffer
/// * Only iterating moves that match a certain pattern
/// * Being iterable multiple times (such as, iterating once for all captures, then iterating again
///   for all quiets)
/// * Doing as little work early on as possible, so that if you are not going to look at every move, the
///   struture moves faster
/// * Being able to iterate pseudo legal moves, while keeping the (nearly) free legality checks in
///   place
///
/// # Examples
///
/// ```
/// use chess::MoveGen;
/// use chess::Board;
/// use chess::EMPTY;
/// use chess::construct;
///
/// // create a board with the initial position
/// let board = Board::default();
///
/// // create an iterable
/// let mut iterable = MoveGen::new_legal(&board);
///
/// // make sure .len() works.
/// assert_eq!(iterable.len(), 20); // the .len() function does *not* consume the iterator
///
/// // lets iterate over targets.
/// let targets = board.color_combined(!board.side_to_move());
/// iterable.set_iterator_mask(*targets);
///
/// // count the number of targets
/// let mut count = 0;
/// for _ in &mut iterable {
///     count += 1;
///     // This move captures one of my opponents pieces (with the exception of en passant)
/// }
///
/// // now, iterate over the rest of the moves
/// iterable.set_iterator_mask(!EMPTY);
/// for _ in &mut iterable {
///     count += 1;
///     // This move does not capture anything
/// }
///
/// // make sure it works
/// assert_eq!(count, 20);
///
/// ```
pub struct MoveGen {
    moves: Vec<BBMove>,
    promotion_index: usize,
    iterator_mask: BitBoard,
    index: usize,
}

impl MoveGen {
    #[inline(always)]
    fn enumerate_moves(board: &Board) -> Vec<BBMove> {
        let checkers = board.checkers;
        let mask = !board.team_mask(board.team_to_move);
        let mut movelist = Vec::<BBMove>::new();

        if checkers.is_empty() {
            PawnType::legals(&mut movelist, &board, mask, false);
            KnightType::legals(&mut movelist, &board, mask, false);
            BishopType::legals(&mut movelist, &board, mask, false);
            RookType::legals(&mut movelist, &board, mask, false);
            QueenType::legals(&mut movelist, &board, mask, false);
            KingType::legals(&mut movelist, &board, mask, false);
        } else if checkers.count_bits() == 1 {
            PawnType::legals(&mut movelist, &board, mask, true);
            KnightType::legals(&mut movelist, &board, mask, true);
            BishopType::legals(&mut movelist, &board, mask, true);
            RookType::legals(&mut movelist, &board, mask, true);
            QueenType::legals(&mut movelist, &board, mask, true);
            KingType::legals(&mut movelist, &board, mask, true);
        } else {
            KingType::legals(&mut movelist, &board, mask, true);
        }

        return movelist;
    }

    /// Create a new `MoveGen` structure, only generating legal moves
    #[inline(always)]
    pub fn new_legal(board: &Board) -> MoveGen {
        MoveGen {
            moves: MoveGen::enumerate_moves(board),
            promotion_index: 0,
            iterator_mask: BitBoard::FULL,
            index: 0,
        }
    }

    /// Never, ever, iterate any moves that land on the following squares
    pub fn remove_mask(&mut self, mask: BitBoard) {
        for x in 0..self.moves.len() {
            self.moves[x].move_mask &= !mask;
        }
    }

    /// Never, ever, iterate this move
    pub fn remove_move(&mut self, chess_move: Move) -> bool {
        for x in 0..self.moves.len() {
            if self.moves[x].origin == chess_move.origin {
                self.moves[x].move_mask &= !BitBoard::from_square(chess_move.destination);
                return true;
            }
        }
        false
    }

    /// For now, Only iterate moves that land on the following squares
    /// Note: Once iteration is completed, you can pass in a mask of ! `EMPTY`
    ///       to get the remaining moves, or another mask
    pub fn set_iterator_mask(&mut self, mask: BitBoard) {
        self.iterator_mask = mask;
        self.index = 0;

        // the iterator portion of this struct relies on the invariant that
        // the bitboards at the beginning of the moves[] array are the only
        // ones used.  As a result, we must partition the list such that the
        // assumption is true.

        // first, find the first non-used moves index, and store that in i
        let mut i = 0;
        while i < self.moves.len() && !(self.moves[i].move_mask & self.iterator_mask).is_empty() {
            i += 1;
        }

        // next, find each element past i where the moves are used, and store
        // that in i.  Then, increment i to point to a new unused slot.
        for j in (i + 1)..self.moves.len() {
            if self.moves[j].move_mask & self.iterator_mask != BitBoard::EMPTY {
                let backup = self.moves[i];
                self.moves[i] = self.moves[j];
                self.moves[j] = backup;
                i += 1;
            }
        }
    }

    /// This function checks the legality *only for moves generated by `MoveGen`*.
    ///
    /// Calling this function for moves not generated by `MoveGen` will result in possibly
    /// incorrect results, and making that move on the `Board` will result in undefined behavior.
    /// This function may panic! if these rules are not followed.
    ///
    /// If you are validating a move from a user, you should call the .legal() function.
    pub fn legal_quick(board: &Board, chess_move: Move) -> bool {
        let piece = board.piece_on(chess_move.origin).unwrap();
        match piece {
            Piece::Rook => true,
            Piece::Bishop => true,
            Piece::Knight => true,
            Piece::Queen => true,
            Piece::Pawn => {
                if chess_move.origin.get_file() != chess_move.destination.get_file()
                    && board.piece_on(chess_move.destination).is_none()
                {
                    // en-passant
                    PawnType::legal_ep_move(board, chess_move.origin, chess_move.destination)
                } else {
                    true
                }
            }
            Piece::King => {
                let bb = magic::rays::between(chess_move.origin, chess_move.destination);
                if bb.count_bits() == 1 {
                    // castles
                    if !KingType::legal_king_move(board, bb.to_square()) {
                        false
                    } else {
                        KingType::legal_king_move(board, chess_move.destination)
                    }
                } else {
                    KingType::legal_king_move(board, chess_move.destination)
                }
            }
        }
    }
}

impl ExactSizeIterator for MoveGen {
    /// Give the exact length of this iterator
    fn len(&self) -> usize {
        let mut result = 0;
        for i in 0..self.moves.len() {
            if self.moves[i].move_mask & self.iterator_mask == BitBoard::EMPTY {
                break;
            }
            if self.moves[i].promotion {
                result += ((self.moves[i].move_mask & self.iterator_mask).count_bits() as usize)
                    * Piece::PROMOTION_TARGETS.len();
            } else {
                result += (self.moves[i].move_mask & self.iterator_mask).count_bits() as usize;
            }
        }
        result
    }
}

impl Iterator for MoveGen {
    type Item = Move;

    /// Give a size_hint to some functions that need it
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    /// Find the next chess move.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.moves.len()
            || self.moves[self.index].move_mask & self.iterator_mask == BitBoard::EMPTY
        {
            // are we done?
            return None;
        } else if self.moves[self.index].promotion {
            let moves = &mut self.moves[self.index];

            let destination = (moves.move_mask & self.iterator_mask).to_square();

            // deal with potential promotions for this pawn
            let result = Move {
                origin: moves.origin,
                destination,
                promotion: Some(Piece::PROMOTION_TARGETS[self.promotion_index]),
            };
            self.promotion_index += 1;
            if self.promotion_index >= Piece::PROMOTION_TARGETS.len() {
                moves.move_mask ^= BitBoard::from_square(destination);
                self.promotion_index = 0;
                if moves.move_mask & self.iterator_mask == BitBoard::EMPTY {
                    self.index += 1;
                }
            }

            return Some(result);
        } else {
            // not a promotion move, so its a 'normal' move as far as this function is concerned
            let moves = &mut self.moves[self.index];
            let dest = (moves.move_mask & self.iterator_mask).to_square();

            moves.move_mask ^= BitBoard::from_square(dest);
            if moves.move_mask & self.iterator_mask == BitBoard::EMPTY {
                self.index += 1;
            }
            
            return Some(Move::new(moves.origin, dest));
        }
    }
}
