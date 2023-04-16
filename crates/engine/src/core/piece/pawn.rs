use crate::core::{board::Square, moves::Move};

use super::{ChessPieceVariant, PieceType};

pub struct PawnType;

impl PieceType for PawnType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Pawn;

    fn pseudo_legal_moves(&self, position: Square, team: crate::core::team::Team) -> Vec<Move> {
        // ! Promotitions are not handled atm
        let mut moves = vec![];

        let destination = position
            .forward(team)
            .expect("A pawn should never be allowed to forward when it's in the promoting rank");

        moves.push(Move::new(Self::PIECE_VARIANT, position, destination));


        if let Ok(destination) = destination.forward(team) {
        moves.push(Move::new(Self::PIECE_VARIANT, position, destination));

        }

        return moves;
    }
}
