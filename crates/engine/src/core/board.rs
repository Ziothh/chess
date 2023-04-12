use super::piece::ChessPiece;

pub type ChessBoardCellValue = Option<ChessPiece>;
pub type ChessBoardCells = [ChessBoardCellValue; ChessBoard::SIZE];

#[derive(Debug)]
pub struct ChessBoard(ChessBoardCells);

impl ChessBoard {
    pub const WIDTH: usize = 8;
    pub const SIZE: usize = ChessBoard::WIDTH * ChessBoard::WIDTH;
    pub const EMPTY_CELL: ChessBoardCellValue = None;

    pub fn new(cells: ChessBoardCells) -> Self {
        Self(cells)
    }

    pub fn set_cell(&mut self, index: usize, value: ChessBoardCellValue) -> &mut Self {
      self.0[index] = value;

      self
    }


    /** Instanciates a `ChessBoard` with empty cells */
    pub fn empty() -> Self {
        ChessBoard::new([ChessBoard::EMPTY_CELL; ChessBoard::SIZE])
    }
}

impl IntoIterator for ChessBoard {
    type Item = ChessBoardCellValue;
    type IntoIter = std::array::IntoIter<ChessBoardCellValue, { ChessBoard::SIZE }>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

