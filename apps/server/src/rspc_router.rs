use engine::core::game::Chess;
use rspc::{Config, Router, RequestLayer};

pub struct MyCtx {}

pub fn router() -> std::sync::Arc<Router<MyCtx>> {
    Router::<MyCtx>::new()
        .config(
            Config::new()
                .set_ts_bindings_header("/* eslint-disable */")
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"),
                ),
        )
        // Define a query taking a string and returning it
        .query("echo", |t| t(|ctx, input: String| input))
        // .query("start", |t| {
        //     t(|ctx, args: ()| async move { Chess::default() })
        // })
        .build()
        .arced()
}
