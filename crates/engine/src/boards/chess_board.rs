use crate::primitives::{board::Square, piece::ChessPiece, team::Team};

use crate::primitives::{File, Rank};
use crate::utils::enums::ArrayEnum;

use serde_big_array::BigArray;

pub type ChessBoardCellValue = Option<ChessPiece>;
pub type ChessBoardCells = [ChessBoardCellValue; ChessBoard::SIZE];
pub type CellIndex = usize;

#[derive(Debug, Clone, Copy, rspc::Type, serde::Serialize, serde::Deserialize)]
pub struct ChessBoard(#[serde(with = "BigArray")] pub [Option<ChessPiece>; ChessBoard::SIZE]);

impl ChessBoard {
    pub const SIZE: usize = File::SIZE * Rank::SIZE;
    pub const EMPTY_CELL: ChessBoardCellValue = None;

    // [Constructor functions]
    /// Creates a `ChessBoard` with the given `cells`
    pub fn new(cells: [Option<ChessPiece>; ChessBoard::SIZE]) -> Self {
        Self(cells)
    }
    /// Creates a `ChessBoard` with all squares being empty
    pub fn empty() -> Self {
        ChessBoard::new([ChessBoard::EMPTY_CELL; ChessBoard::SIZE])
    }

    // [Instance methods]
    // [[Square methods]]
    pub fn get(&self, square: Square) -> &ChessBoardCellValue {
        self.0
            .get(square.to_index())
            .expect("square.to_index() to be in range 0..64")
    }
    pub fn set(&mut self, square: Square, piece: ChessPiece) -> &mut Self {
        self.0[square.to_index()] = Some(piece);
        return self;
    }
    pub fn remove(&mut self, square: Square) -> &mut Self {
        self.0[square.to_index()] = None;
        return self;
    }
    /**
       Swaps the values of cell indices `a` and `b`.
      `a <-> b`

      TODO: TEST THIS
    */
    pub fn swap(&mut self, a: Square, b: Square) -> &mut Self {
        self.0.swap(a.to_index(), b.to_index());
        return self;
    }


    pub fn iter(&self) -> impl Iterator<Item = &ChessBoardCellValue> {
        self.0.iter()
    }
    pub fn iter_team(&self, team: Team) -> impl Iterator<Item = (Square, &ChessPiece)> {
        self.iter().enumerate().filter_map(move |(i, cell)| {
            let Some(piece) = cell else { return None; };

            if piece.team != team {
                return None;
            }

            Some((Square::new(i as u8), piece))
        })
    }
}
