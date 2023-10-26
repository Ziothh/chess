use crate::router::prelude::{Router, R};

mod chess;
mod prelude;

pub use prelude::MyCtx;

pub fn create() -> std::sync::Arc<Router> {
    R.router()
        // .middleware(|mw| mw.middleware(|mw| async move {
        //         let state = (mw.req.clone(), mw.ctx.clone(), mw.input.clone());
        //         Ok(mw.with_state(state))
        //     })
        //     .resp(|state, result| async move {
        //         println!(
        //             "[LOG] req='{:?}' ctx='{:?}'  input='{:?}' result='{:?}'",
        //             state.0, state.1, state.2, result
        //         );
        //         Ok(result)
        //     })
        // )
        // Define a query taking a string and returning it
        .procedure("echo", R.query(|_ctx, input: String| Ok(input)))
        .merge("chess", chess::router())
        .build()
        .unwrap()
        .arced()
}
