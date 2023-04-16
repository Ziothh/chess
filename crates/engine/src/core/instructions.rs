use crate::core::team::Team;

use super::moves::Move;

#[derive(Debug, PartialEq)]
pub enum DirectionOffset {
    N = 8isize,
    S = -8isize,
    W = -1isize,
    E = 1isize,
    NW = 7isize,
    SE = -7isize,
    NE = 9isize,
    SW = -9isize,
}

#[derive(Debug, PartialEq)]
pub enum CastlingMove {
    QueenSide,
    KingSide,
}

#[derive(Debug, PartialEq)]
pub enum EndOfGameState {
    Draw,
    Win(Team),
}

#[derive(Debug, PartialEq)]
pub enum CheckKind {
    Check,
    Mate,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Castling(CastlingMove),
    EndOfGame(EndOfGameState),
    Move(Move),
}
