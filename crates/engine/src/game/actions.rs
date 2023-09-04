use crate::primitives::team::Team;

use super::moves::Move;

// #[derive(Debug, PartialEq)]
// pub enum CastlingMove {
//     QueenSide,
//     KingSide,
// }

// #[derive(Debug, rspc::Type, serde::Serialize, PartialEq)]
// pub enum GameState {
//     Ongoingj
// }

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EndOfGameState {
    Draw,
    Stalemate,
    /// The `Team` that checkmated the other and thus won
    Checkmate(Team),
    /// The `Team` that resigned and thus lost
    Resignation(Team),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CheckKind {
    Check,
    Mate,
}

/// Contains all actions supported within the game
#[derive(Debug, rspc::Type, serde::Serialize, PartialEq)]
pub enum Action {
    Move(Move),
    OfferDraw(Team),
    AcceptDraw,
    DeclareDraw,
    Resign(Team),

    // Castling(CastlingMove),
    // EndOfGame(EndOfGameState),
    // Move(Move),
}
