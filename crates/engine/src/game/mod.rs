use std::fmt::Display;

use crate::notations::FEN;

use self::actions::Action;

pub mod actions;
pub mod moves;
pub use moves::Move;
mod board;
pub use board::*; 

// #[derive(Debug, rspc::Type, serde::Serialize)]
pub struct Chess {
    // /// The starting `ChessBoard` state
    // pub starting_position: ChessBoard,
    //
    // /// The history of all actions taken in the current chess game
    // pub history: Vec<Action>,
    
    /// The array of chessboard cells.
    pub board: Board,

    // /// The team to move
    // pub team_to_move: Team,
    // 
    // /// Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
    // pub halfmove_clock: u8,
    // // Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
    // pub fullmove_clock: u8,

    // pub en_passant: Option<Square>,
}

impl Chess {
    // pub fn new(board_pieces: []) -> Self {
    //     todo!()
    // }

    pub fn new_from_history<const N: usize>(history: [Action; N]) -> Self {
        let mut starting_position = Self::default();

        todo!();

        // for x in history.iter() {
        //     match *x {
        //         Action::Move(m) => {
        //             todo!()
        //             // starting_position.make_move(m);
        //         }
        //         _ => {}
        //     }
        // }

        return starting_position;
    }

    // [Getter methods]
    // pub fn team_to_move(&self)


    // /// Returns the side to move, based on `self.halfmove_clock`
    // pub fn team_to_move(&self) -> Team {
    //     if self.halfmove_clock % 2 == 0 {
    //         Team::White
    //     } else {
    //         Team::Black
    //     }
    // // }

    // pub fn make_move(&mut self, origin: Square, destination: Square) -> Result<&Self, String> {
    //     if self
    //         .generate_legal_moves()
    //         .iter()
    //         .find(|m| m.origin == origin && m.destination == destination)
    //         .is_none()
    //     {
    //         return Err(format!("Move {origin} => {destination} could not be found"));
    //     }
    //
    //     self.board.swap(origin, destination).remove(origin);
    //
    //     self.team_to_move = match self.team_to_move {
    //         Team::Black => Team::White,
    //         Team::White => Team::Black,
    //     };
    //
    //     return Ok(self);
    // }
    //
    // pub fn generate_legal_moves(&self) -> Vec<Move> {
    //     self.board.generate_legal_moves(self.team_to_move)
    // }
    //
    // pub fn follow_instruction(&self, instruction: Instruction) -> &Self {
    //     todo!();
    //     self
    // }
}

impl Default for Chess {
    fn default() -> Self {
        FEN::gamestate_from_fen(FEN::START).unwrap()
    }
}

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // write!(
        //     f,
        //     "{}",
        //     self.board
        //         .iter()
        //         .enumerate()
        //         .map(|(i, cell)| {
        //             let mut string_repr = if let Some(piece) = cell {
        //                 piece.to_string()
        //             } else {
        //                 ".".to_string()
        //             };
        //
        //             if i % 8 == 0 && i != 0 {
        //                 string_repr.insert_str(0, "\n");
        //             }
        //
        //             return string_repr;
        //         })
        //         .collect::<Vec<String>>()
        //         .join(" ")
        // )
    }
}
