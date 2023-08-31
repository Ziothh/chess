use std::str::FromStr;

use engine::primitives::{
    board::{ChessBoard, Square},
    game::Chess,
    // moves::Move,
    piece::ChessPieceVariant,
    team::Team,
};
use rspc::{Config, Router, RouterBuilder};
// use serde::Serialize; // This requires the 'derive' feature to be enabled.

pub struct MyCtx {}

pub fn router() -> std::sync::Arc<Router<MyCtx>> {
    Router::<MyCtx>::new()
        .config(
            Config::new()
                .set_ts_bindings_header("/* eslint-disable */")
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./ts/bindings.ts"),
                ),
        )
        // Define a query taking a string and returning it
        .query("echo", |t| t(|_ctx, input: String| input))
        .merge("chess.", chess_router())
        .build()
        .arced()
}

fn chess_router() -> RouterBuilder<MyCtx> {
    Router::<MyCtx>::new()
        .query("start", |t| {
            t(|_ctx, _args: ()| ChessJSON::from(Chess::default()))
        })
        .mutation("move", |t| {
            t(|_ctx, args: (MoveJSON, ChessJSON)| {
                let (move_data, chess_data) = args;

                let mut chess = Chess::default();
                chess.board = chess_data.board;
                chess.team_to_move = chess_data.team_to_move;

                chess
                    .make_move(
                        Square::from_str(&move_data.origin).unwrap(),
                        Square::from_str(&move_data.destination).unwrap(),
                    )
                    .unwrap();

                return ChessJSON::from(chess);
            })
        })

        // TODO: fen router
        .merge("fen.", Router::<MyCtx>::new())
}

#[derive(Debug, rspc::Type, serde::Serialize, serde::Deserialize)]
struct MoveJSON {
    origin: String,
    destination: String,
    takes: bool,
    piece: ChessPieceVariant,
}

#[derive(Debug, rspc::Type, serde::Serialize, serde::Deserialize)]
struct ChessJSON {
    #[serde(rename = "teamToMove")]
    team_to_move: Team,
    moves: Vec<MoveJSON>,
    board: ChessBoard,
}

impl From<Chess> for ChessJSON {
    fn from(value: Chess) -> Self {
        Self {
            team_to_move: value.team_to_move,
            moves: value
                .generate_legal_moves()
                .iter()
                .map(|m| MoveJSON {
                    piece: m.piece,
                    origin: m.origin.to_string(),
                    destination: m.destination.to_string(),
                    takes: m.takes,
                })
                .collect(),
            board: value.board,
        }
    }
}
