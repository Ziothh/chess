use crate::core::{board::Square, moves::Move, piece::ChessPiece, team::Team};

use super::{NUM_FILES, NUM_RANKS};

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
    pub fn get(&self, index: CellIndex) -> &ChessBoardCellValue {
        self.0.get(index).expect("index to be in range 0..64")
    }

    pub fn set(&mut self, index: CellIndex, value: ChessBoardCellValue) -> &mut Self {
        self.0[index] = value;
        return self;
    }
    pub fn set_piece(&mut self, index: CellIndex, piece: ChessPiece) -> &mut Self {
        return self.set(index, Some(piece));
    }

    pub fn iter(&self) -> impl Iterator<Item = &ChessBoardCellValue> {
        self.0.iter()
    }

    /**
       Swaps the values of cell indices `a` and `b`.
      `a <-> b`

      TODO: TEST THIS
    */
    pub fn swap(&mut self, a: CellIndex, b: CellIndex) -> &mut Self {
        let (left, right) = self.0.split_at_mut(b);

        std::mem::swap(&mut left[a], &mut right[0]);

        self
    }

    pub fn group_by_team(&self, team: Team) -> impl Iterator<Item = &ChessPiece> {
        self.iter().filter_map(move |cell| {
            let Some(piece) = cell else { return None; };

            if piece.team != team {
                return None;
            }

            Some(piece)
        })
    }

    pub fn generate_legal_moves(&self, team_to_move: Team) -> Vec<Move> {
        self.group_by_team(team_to_move)
            .map(|piece| {
                let moves: Vec<Move> = Vec::new();

                moves
            })
            .flatten()
            .collect()
    }

    /// TODO: flip the ranks (1 at bottom)
    pub fn print_ascii(&self) -> &Self {
        self.0.iter().enumerate().for_each(|(i, cell)| {
            let index_str = if let Some(piece) = cell {
                piece.clone().to_string()
            } else {
                ".".to_string()
            };

            let spacer = if Square::new(i as u8).get_file().to_index() == 7 {
                "\n"
            } else {
                " "
            };

            print!("{index_str}{spacer}");
        });

        return self;
    }
}

// impl IntoIterator for ChessBoard {
//     type Item = ChessBoardCellValue;
//     type IntoIter = std::array::IntoIter<ChessBoardCellValue, { ChessBoard::SIZE }>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
