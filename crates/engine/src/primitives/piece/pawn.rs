use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{ChessPieceVariant, PieceType},
    team::Team,
};

pub struct PawnType;

impl PieceType for PawnType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Pawn;

    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move> {
        // ! Promotitions are not handled atm
        let mut moves = vec![];

        let destination = position
            .forward(team)
            .expect("A pawn should never be allowed to forward when it's in the promoting rank");

        moves.push(Move::new(Self::PIECE_VARIANT, position, destination));

        // The starting 2 step jump allowing for en passant
        if position.get_rank() == team.get_nth_rank(2) {
            if let Some(destination) = destination.forward(team) {
                moves.push(Move::new(Self::PIECE_VARIANT, position, destination));
            }
        }

        return moves;
    }
}
