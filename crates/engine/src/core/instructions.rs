use crate::core::{piece::ChessPieceVariant, team::Team};

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
pub enum CaslingMove {
    QueenSide,
    KingSide,
}

#[derive(Debug, PartialEq)]
pub enum EndOfGameState {
    Draw,
    Win(Team),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Castling(CaslingMove),
    EndOfGame(EndOfGameState),
    Move(Move),
}

#[derive(Debug, PartialEq)]
pub struct Move {
    /// The original board square index
    origin: u32,
    /// The destination board square index
    destination: u32,
    takes: bool,
    piece: ChessPieceVariant,
}
