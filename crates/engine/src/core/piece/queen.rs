use crate::core::{board::Square, team::Team, moves::Move, piece::{PieceType, ChessPieceVariant}};

pub struct QueenType;

impl PieceType for QueenType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Queen;

    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move> {
        // let moves: Vec<_> = Vec::new();

        // return moves;
        todo!()
    }
}
