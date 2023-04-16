use std::fmt::Display;

use crate::notations::FEN;

use super::{
    board::ChessBoard,
    instructions::{Instruction, Move},
    team::Team,
};

#[derive(Debug)]
pub struct Chess {
    /// The array of chessboard cells.
    pub board: ChessBoard,

    /// The team to move
    pub current_team: Team,
    ///
    /// Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
    pub halfmove_clock: usize,
    // Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
    pub fullmove_clock: usize,
}

impl Chess {
    pub fn new() -> Self {
        todo!()
    }

    /// Returns the side to move, based on `self.halfmove_clock`
    pub fn team_to_move(&self) -> Team {
        if self.halfmove_clock % 2 == 0 {
            Team::White
        } else {
            Team::Black
        }
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.board.generate_legal_moves(self.team_to_move())
    }

    pub fn follow_instruction(&self, instruction: Instruction) -> Self {
      self
    }
}

impl Default for Chess {
    fn default() -> Self {
        FEN::gamestate_from_fen(FEN::START).unwrap()
    }
}

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .into_iter()
                .enumerate()
                .map(|(i, cell)| {
                    let mut string_repr = if let Some(piece) = cell {
                        piece.to_string()
                    } else {
                        ".".to_string()
                    };

                    if i % 8 == 0 && i != 0 {
                        string_repr.insert_str(0, "\n");
                    }

                    return string_repr;
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
