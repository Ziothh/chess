use crate::{
    bitboard::BitBoard,
    boards::Board,
    magic,
    primitives::{Piece, Square, Team},
};


#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct BBMove {
    pub origin: Square,
    pub move_mask: BitBoard,
    pub promotion: bool,
}

pub trait PieceMovegen {
    fn is(piece: Piece) -> bool;
    fn into_piece() -> Piece;
    fn pseudo_legals(origin: Square, team: Team, combined: BitBoard, mask: BitBoard) -> BitBoard;
    fn legals(movelist: &mut Vec<BBMove>, board: &Board, mask: BitBoard, in_check: bool) {
        let combined = board.all_mask();
        let team = board.team_to_move;
        let my_pieces = board.team_mask(team);
        let ksq = board.king_square(team);

        let pieces = board.piece_mask(Self::into_piece()) & my_pieces;
        let pinned = board.pinned;
        let checkers = board.checkers;

        let check_mask = match in_check {
            true => magic::rays::between(checkers.to_square(), ksq) ^ checkers,
            false => !BitBoard::EMPTY,
        };

        for origin in pieces & !pinned {
            let moves = Self::pseudo_legals(origin, team, combined, mask) & check_mask;
            if !moves.is_empty() {
                movelist.push(BBMove {
                    origin,
                    move_mask: moves,
                    promotion: false,
                });
            }
        }

        if !in_check {
            for origin in pieces & pinned {
                let moves = Self::pseudo_legals(origin, team, combined, mask)
                    & magic::rays::line(origin, ksq);
                if !moves.is_empty() {
                    movelist.push(BBMove {
                        origin,
                        move_mask: moves,
                        promotion: false,
                    })
                }
            }
        }
    }
}

pub struct PawnType;
pub struct BishopType;
pub struct KnightType;
pub struct RookType;
pub struct QueenType;
pub struct KingType;

impl PawnType {
    /// Is a particular en-passant capture legal?
    pub fn legal_ep_move(board: &Board, source: Square, dest: Square) -> bool {
        let combined = board.all_mask()
            ^ BitBoard::from_square(board.en_passant.unwrap())
            ^ BitBoard::from_square(source)
            ^ BitBoard::from_square(dest);

        let ksq = (board.piece_mask(Piece::King) & board.team_mask(board.team_to_move)).to_square();

        let rooks = (board.piece_mask(Piece::Rook) | board.piece_mask(Piece::Queen))
            & board.team_mask(!board.team_to_move);

        if (magic::rook::get_rays(ksq) & rooks) != BitBoard::EMPTY {
            if (magic::rook::get_moves(ksq, combined) & rooks) != BitBoard::EMPTY {
                return false;
            }
        }

        let bishops = (board.piece_mask(Piece::Bishop) | board.piece_mask(Piece::Queen))
            & board.team_mask(!board.team_to_move);

        if (magic::bishop::get_rays(ksq) & bishops) != BitBoard::EMPTY {
            if (magic::bishop::get_moves(ksq, combined) & bishops) != BitBoard::EMPTY {
                return false;
            }
        }

        return true;
    }
}

impl PieceMovegen for PawnType {
    fn is(piece: Piece) -> bool {
        piece == Piece::Pawn
    }

    fn into_piece() -> Piece {
        Piece::Pawn
    }

    #[inline(always)]
    fn pseudo_legals(origin: Square, team: Team, combined: BitBoard, mask: BitBoard) -> BitBoard {
        magic::pawn::get_moves(origin, team, combined) & mask
    }

    #[inline(always)]
    fn legals(movelist: &mut Vec<BBMove>, board: &Board, mask: BitBoard, in_check: bool) {
        let combined = board.all_mask();
        let team = board.team_to_move;
        let my_pieces = board.team_mask(team);
        let ksq = board.king_square(team);

        let pieces = board.piece_mask(Self::into_piece()) & my_pieces;
        let pinned = board.pinned;
        let checkers = board.checkers;

        let check_mask = match in_check {
            true => magic::rays::between(checkers.to_square(), ksq) ^ checkers,
            false => !BitBoard::EMPTY,
        };

        for origin in pieces & !pinned {
            let moves = Self::pseudo_legals(origin, team, combined, mask) & check_mask;
            if !moves.is_empty() {
                movelist.push(BBMove {
                    origin,
                    move_mask: moves,
                    promotion: origin.get_rank() == team.get_nth_rank(7),
                });
            }
        }

        if !in_check {
            for origin in pieces & pinned {
                let moves = Self::pseudo_legals(origin, team, combined, mask)
                    & magic::rays::line(ksq, origin);
                if !moves.is_empty() {
                    movelist.push(BBMove {
                        origin,
                        move_mask: moves,
                        promotion: origin.get_rank() == team.get_nth_rank(7),
                    });
                }
            }
        }

        if let Some(en_passant_square) = board.en_passant {
            let rank_mask = BitBoard::mask_rank(en_passant_square.get_rank());
            let file_mask = BitBoard::mask_adjacent_files(en_passant_square.get_file());
            for origin in rank_mask & file_mask & pieces {
                let dest = en_passant_square.uforward(team);
                if PawnType::legal_ep_move(board, origin, dest) {
                    movelist.push(BBMove {
                        origin,
                        move_mask: BitBoard::from_square(dest),
                        promotion: false,
                    });
                }
            }
        }
    }
}

impl PieceMovegen for BishopType {
    fn is(piece: Piece) -> bool {
        piece == Piece::Bishop
    }

    fn into_piece() -> Piece {
        Piece::Bishop
    }

    #[inline(always)]
    fn pseudo_legals(src: Square, _team: Team, combined: BitBoard, mask: BitBoard) -> BitBoard {
        magic::bishop::get_moves(src, combined) & mask
    }
}

impl PieceMovegen for KnightType {
    fn is(piece: Piece) -> bool {
        piece == Piece::Knight
    }

    fn into_piece() -> Piece {
        Piece::Knight
    }

    #[inline(always)]
    fn pseudo_legals(origin: Square, _team: Team, _combined: BitBoard, mask: BitBoard) -> BitBoard {
        magic::knight::get_moves(origin) & mask
    }

    #[inline(always)]
    fn legals(movelist: &mut Vec<BBMove>, board: &Board, mask: BitBoard, in_check: bool) {
        let combined = board.all_mask();
        let team = board.team_to_move;
        let my_pieces = board.team_mask(team);
        let ksq = board.king_square(team);

        let pieces = board.piece_mask(Self::into_piece()) & my_pieces;
        let pinned = board.pinned;
        let checkers = board.checkers;

        if in_check {
            let check_mask = magic::rays::between(checkers.to_square(), ksq) ^ checkers;

            for origin in pieces & !pinned {
                let moves = Self::pseudo_legals(origin, team, combined, mask & check_mask);
                if !moves.is_empty() {
                    movelist.push(BBMove {
                        origin,
                        move_mask: moves,
                        promotion: false,
                    });
                }
            }
        } else {
            for origin in pieces & !pinned {
                let moves = Self::pseudo_legals(origin, team, combined, mask);
                if !moves.is_empty() {
                    movelist.push(BBMove {
                        origin,
                        move_mask: moves,
                        promotion: false,
                    });
                }
            }
        };
    }
}

impl PieceMovegen for RookType {
    fn is(piece: Piece) -> bool {
        piece == Piece::Rook
    }

    fn into_piece() -> Piece {
        Piece::Rook
    }

    #[inline(always)]
    fn pseudo_legals(origin: Square, _team: Team, combined: BitBoard, mask: BitBoard) -> BitBoard {
        magic::rook::get_moves(origin, combined) & mask
    }
}

impl PieceMovegen for QueenType {
    fn is(piece: Piece) -> bool {
        piece == Piece::Queen
    }

    fn into_piece() -> Piece {
        Piece::Queen
    }

    #[inline(always)]
    fn pseudo_legals(src: Square, _team: Team, combined: BitBoard, mask: BitBoard) -> BitBoard {
        (magic::rook::get_moves(src, combined) ^ magic::bishop::get_moves(src, combined)) & mask
    }
}

impl KingType {
    /// Is a particular king move legal?
    #[inline(always)]
    pub fn legal_king_move(board: &Board, dest: Square) -> bool {
        let combined = board.all_mask()
            ^ (board.piece_mask(Piece::King) & board.team_mask(board.team_to_move))
            | BitBoard::from_square(dest);

        let mut attackers = BitBoard::EMPTY;

        let rooks = (board.piece_mask(Piece::Rook) | board.piece_mask(Piece::Queen))
            & board.team_mask(!board.team_to_move);

        attackers |= magic::rook::get_moves(dest, combined) & rooks;

        let bishops = (board.piece_mask(Piece::Bishop) | board.piece_mask(Piece::Queen))
            & board.team_mask(!board.team_to_move);

        attackers |= magic::bishop::get_moves(dest, combined) & bishops;

        let knight_rays = magic::knight::get_moves(dest);
        attackers |=
            knight_rays & board.piece_mask(Piece::Knight) & board.team_mask(!board.team_to_move);

        let king_rays = magic::king::get_moves(dest);
        attackers |=
            king_rays & board.piece_mask(Piece::King) & board.team_mask(!board.team_to_move);

        attackers |= magic::pawn::get_attacks(
            dest,
            board.team_to_move,
            board.piece_mask(Piece::Pawn) & board.team_mask(!board.team_to_move),
        );

        return attackers.is_empty();
    }
}

impl PieceMovegen for KingType {
    fn is(piece: Piece) -> bool {
        piece == Piece::King
    }

    fn into_piece() -> Piece {
        Piece::King
    }

    #[inline(always)]
    fn pseudo_legals(origin: Square, _team: Team, _combined: BitBoard, mask: BitBoard) -> BitBoard {
        magic::king::get_moves(origin) & mask
    }

    #[inline(always)]
    fn legals(movelist: &mut Vec<BBMove>, board: &Board, mask: BitBoard, in_check: bool) {
        let combined = board.all_mask();
        let color = board.team_to_move;
        let ksq = board.king_square(color);

        let mut moves = Self::pseudo_legals(ksq, color, combined, mask);

        let copy = moves;
        for dest in copy {
            if !KingType::legal_king_move(board, dest) {
                moves ^= BitBoard::from_square(dest);
            }
        }

        // If we are not in check, we may be able to castle.
        // We can do so iff:
        //  * the `Board` structure says we can.
        //  * the squares between my king and my rook are empty.
        //  * no enemy pieces are attacking the squares between the king, and the kings
        //    destination square.
        //  ** This is determined by going to the left or right, and calling
        //     'legal_king_move' for that square.
        if !in_check {
            if board.castle_rights(board.team_to_move).has_kingside()
                && (combined
                    & board
                        .castle_rights(board.team_to_move)
                        .kingside_squares(color))
                    == BitBoard::EMPTY
            {
                let middle = ksq.uright();
                let right = middle.uright();
                if KingType::legal_king_move(board, middle)
                    && KingType::legal_king_move(board, right)
                {
                    moves ^= BitBoard::from_square(right);
                }
            }

            if board.castle_rights(board.team_to_move).has_queenside()
                && (combined
                    & board
                        .castle_rights(board.team_to_move)
                        .queenside_squares(color))
                    == BitBoard::EMPTY
            {
                let middle = ksq.uleft();
                let left = middle.uleft();
                if KingType::legal_king_move(board, middle)
                    && KingType::legal_king_move(board, left)
                {
                    moves ^= BitBoard::from_square(left);
                }
            }
        }
        if !moves.is_empty() {
            movelist.push(BBMove {
                origin: ksq,
                move_mask: moves,
                promotion: false,
            })
        }
    }
}
