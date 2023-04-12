use crate::piece::ChessPieceVariant;

pub struct Move {
    origin: u32,
    destination: u32,
    takes: bool,
    piece: ChessPieceVariant,
}

// pub struct Move {
//     /// The starting board square index
//     start: usize,
//     /// The destination board square index
//     destination: usize,
//     piece: ChessPiece,
//     // takes: bool;
// }
