use super::{super::{instructions::Move, piece::ChessPiece, team::Team}, NUM_FILES, NUM_RANKS};

pub type ChessBoardCellValue = Option<ChessPiece>;
pub type ChessBoardCells = [ChessBoardCellValue; ChessBoard::SIZE];
pub type CellIndex = usize;

#[derive(Debug)]
pub struct ChessBoard(ChessBoardCells);

impl ChessBoard {
    pub const SIZE: usize = NUM_FILES * NUM_RANKS;
    pub const EMPTY_CELL: ChessBoardCellValue = None;

    pub fn new(cells: ChessBoardCells) -> Self {
        Self(cells)
    }

    /** Instanciates a `ChessBoard` with empty cells */
    pub fn empty() -> Self {
        ChessBoard::new([ChessBoard::EMPTY_CELL; ChessBoard::SIZE])
    }

    // Instance methods ---------------
    pub fn get(&self, index: CellIndex) -> ChessBoardCellValue {
        self.0.get(index)
    }

    pub fn set(&mut self, index: CellIndex, value: ChessBoardCellValue) -> &mut Self {
        self.0[index] = value;

        self
    }

    /**
       Swaps the values of cell indices `a` and `b`.
      `a <-> b`
    */
    pub fn swap(&mut self, a: CellIndex, b: CellIndex) -> &mut Self {
        let val_b = self.get(b);

        self.set(b, self.get(a));
        self.set(a, val_b);

        self
    }

    pub fn group_by_team(&self, team: Team) -> impl Iterator<Item = ChessPiece> {
        let x = self.into_iter().filter_map(|cell| {
            let Some(piece) = cell else { return None; };

            if piece.team != team {
                return None;
            }

            Some(piece)
        });
    }

    pub fn generate_legal_moves(&self, team_to_move: Team) -> Vec<Move> {
        self.group_by_team(team_to_move)
            .map(|x| {
                let moves: Vec<Move>;

                moves
            })
            .flatten()
    }
}

impl IntoIterator for ChessBoard {
    type Item = ChessBoardCellValue;
    type IntoIter = std::array::IntoIter<ChessBoardCellValue, { ChessBoard::SIZE }>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
