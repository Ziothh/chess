use std::fmt::Display;

use crate::notations::FEN;

use super::{
    board::{ChessBoard, Square},
    instructions::Instruction,
    moves::Move,
    team::Team,
};

#[derive(Debug)]
#[derive(rspc::Type, serde::Serialize)]
pub struct Chess {
    /// The array of chessboard cells.
    pub board: ChessBoard,

    /// The team to move
    pub team_to_move: Team,
    ///
    /// Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
    pub halfmove_clock: u8,
    // Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
    pub fullmove_clock: u8,
}

impl Chess {
    pub fn new() -> Self {
        todo!()
    }

    // /// Returns the side to move, based on `self.halfmove_clock`
    // pub fn team_to_move(&self) -> Team {
    //     if self.halfmove_clock % 2 == 0 {
    //         Team::White
    //     } else {
    //         Team::Black
    //     }
    // }
    pub fn make_move(&mut self, origin: Square, destination: Square) -> Result<&Self, String> {
        if self
            .generate_legal_moves()
            .iter()
            .find(|m| m.origin == origin && m.destination == destination)
            .is_none()
        {
            return Err(format!("Move {origin} => {destination} could not be found"));
        }

        self.board.swap(origin, destination).remove(origin);

        self.team_to_move = match self.team_to_move {
            Team::Black => Team::White,
            Team::White => Team::Black,
        };

        return Ok(self);
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.board.generate_legal_moves(self.team_to_move)
    }

    pub fn follow_instruction(&self, instruction: Instruction) -> &Self {
        todo!();
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
                .iter()
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
