use crate::{
    piece::{ChessPiece, ChessPieceVariant},
    team::Team,
};

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

pub enum CaslingMove {
    QueenSide,
    KingSide,
}

pub enum EndOfGameState {
    Draw,
    Win(Team),
}

pub struct 

pub enum Instruction {
    Castling(CaslingMove),
    EndOfGame(EndOfGameState),
    Move {
        origin: u32,
        destination: u32,
        takes: bool,
        piece: ChessPieceVariant,
    },
}
