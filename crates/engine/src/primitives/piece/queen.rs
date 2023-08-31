use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{ChessPieceVariant, PieceType},
    team::Team,
};

use super::{BishopType, RookType};

pub struct QueenType;

impl PieceType for QueenType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Queen;

    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move> {
        let mut moves = RookType::pseudo_legal_moves(position, team);
        moves.append(&mut BishopType::pseudo_legal_moves(position, team));
        return moves;
    }
}
