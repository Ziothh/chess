use engine::core::{board::ChessBoard, game::Chess, moves::Move, team::Team};
use rspc::{Config, Router, Type, RouterBuilder};
use serde::Serialize; // This requires the 'derive' feature to be enabled.

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
        .query("start", |t| t(|_ctx, _args: ()| Chess::default()))
}
// struct ChessJSON {
//   team_to_move: Team,
//   moves: Vec<Move>,
//   board: ChessBoard,
//   test: bool
// }
//
// impl From<Chess> for ChessJSON {
//   fn from(value: Chess) -> Self {
//
//   }
// }
