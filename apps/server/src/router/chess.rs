use std::str::FromStr;

use super::prelude::{RouterBuilder, R};
use engine::{
    boards::ChessBoard,
    game::{Board, BoardStatus, Move},
    primitives::{ChessPiece, Piece, Square, Team},
};

pub fn router() -> RouterBuilder {
    R.router()
        .procedure(
            "start",
            R.query(|_ctx, _args: ()| Ok(ChessJSON::from(Board::default()))),
        )
        .procedure(
            "move",
            R.mutation(|_ctx, (move_data, chess_state): (MoveJSON, ChessJSON)| {
                let mut board: Board = chess_state.into();

                println!("Board before move: \n{board}");

                board.make_move(
                    move_data
                    .try_into()
                    // TODO: find a way to do map_err
                    .unwrap()
                    // .map_err(|err| {
                    //     rspc::Error::new(rspc::ErrorCode::InternalServerError, err)
                    // })?
                );

                println!("Board after move: \n{board}");

                return Ok(ChessJSON::from(board));
            }),
        )
        // TODO: fen router
        .merge("fen", R.router())
}

#[derive(Debug, specta::Type, serde::Serialize, serde::Deserialize)]
struct MoveJSON {
    origin: String,
    destination: String,
    takes: bool,
    piece: Piece,
    promotion: Option<Piece>,
}

impl TryInto<Move> for MoveJSON {
    type Error = String;
    fn try_into(self) -> Result<Move, Self::Error> {
        Ok(Move {
            origin: Square::from_str(&self.origin)
                .map_err(|_| format!("Invalid move origin square string \"{}\"", self.origin))?,
            destination: Square::from_str(&self.destination).map_err(|_| {
                format!("Invalid move destination square string \"{}\"", self.origin)
            })?,
            promotion: self.promotion,
        })
    }
}

impl MoveJSON {
    pub fn from_move(board: Board, chess_move: Move) -> Self {
        Self {
            origin: chess_move.origin.to_string(),
            destination: chess_move.destination.to_string(),
            promotion: chess_move.promotion,
            takes: board
                .team_on(chess_move.destination)
                .is_some_and(|team| team != board.team_to_move),
            piece: board
                .piece_on(chess_move.origin)
                .expect("Move origin to contain a piece"),
        }
    }
}

#[derive(Debug, specta::Type, serde::Serialize, serde::Deserialize)]
struct ChessJSON {
    #[serde(rename = "teamToMove")]
    team_to_move: Team,
    moves: Vec<MoveJSON>,
    board: ChessBoard,
    status: BoardStatus,
}

impl From<Board> for ChessJSON {
    fn from(board: Board) -> Self {
        Self {
            status: board.status(),
            moves: board
                .iter_moves()
                .map(|m| MoveJSON::from_move(board, m))
                .collect(),
            team_to_move: board.team_to_move,
            board: {
                let mut chess_board = ChessBoard::empty();

                for square in (0..Board::SIZE as u8).map(|index| Square::new(index)) {
                    if let Some(piece) = board.piece_on(square) && let Some(team) = board.team_on(square) {
                        chess_board.set(square, ChessPiece {
                            team,
                            variant: piece,
                        });
                    }
                }

                chess_board
            },
        }
    }
}

impl Into<Board> for ChessJSON {
    fn into(self) -> Board {
        let mut board = Board::new(self.board.0);
        board.team_to_move = self.team_to_move;

        return board;
    }
}

// TODO: fix this
// impl From<Chess> for ChessJSON {
//     fn from(value: Chess) -> Self {
//         Self {
//             team_to_move: value.team_to_move,
//             moves: value
//                 .generate_legal_moves()
//                 .iter()
//                 .map(|m| MoveJSON {
//                     piece: m.piece,
//                     origin: m.origin.to_string(),
//                     destination: m.destination.to_string(),
//                     takes: m.takes,
//                 })
//                 .collect(),
//             board: value.board,
//         }
//     }
// }
