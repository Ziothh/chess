use crate::core::board::{Square, NUM_FILES, NUM_RANKS};

use super::ChessBoard;

pub struct ASCIIBoard {
    // pub board: ChessBoard,
    pub marks: [bool; ChessBoard::SIZE],
}

impl ASCIIBoard {
    const DEFAULT_MARKS: [bool; ChessBoard::SIZE] = [false; ChessBoard::SIZE];

    pub fn new() -> Self {
        Self {
            // board,
            marks: Self::DEFAULT_MARKS,
        }
    }

    pub fn print(&self, board: &ChessBoard) -> &Self {
        let mut ranks: [String; NUM_RANKS + 1] = Default::default();

        board.iter().enumerate().for_each(|(i, cell)| {
            let (file, rank) = Square::new(i as u8).to_coords();

            let file_index = file.to_index();
            let rank_index = rank.to_index();

            // Add file legend
            if rank_index == 0 {
                let rank_str = ranks.get_mut(0).unwrap();
                if file_index == 0 {
                    rank_str.push_str("  ")
                }
                rank_str.push_str(&format!("{} ", file.to_char()))
            }

            let rank_str = ranks.get_mut(rank_index + 1).unwrap();

            // Add rank legend
            if file_index == 0 {
                rank_str.push_str(&format!("{} ", rank_index + 1))
            }

            let index_str = if let Some(piece) = cell {
                piece.clone().to_string()
            } else if *self.marks.get(i).unwrap() {
                "x".to_string()
            } else {
                ".".to_string()
            };

            rank_str.push_str(&format!("{index_str} "));
        });

        ranks.reverse();

        println!("{}", ranks.join("\n"));

        return self;
    }

    pub fn clear_marks(&mut self) -> &mut Self {
        self.marks = Self::DEFAULT_MARKS;
        return self;
    }
    pub fn mark(&mut self, square: Square) -> &mut Self {
        self.marks[square.to_index()] = true;
        return self;
    }
    pub fn unmark(&mut self, square: Square) -> &mut Self {
        self.marks[square.to_index()] = false;
        return self;
    }
}