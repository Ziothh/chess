use std::net::SocketAddr;

use axum::routing::{get, post};

mod rspc_router;
use crate::rspc_router::{router, MyCtx};

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> () {
    let app = axum::Router::new()
        .route("/", get(|| async { "Spacedrive Server!" }))
        .route("/health", get(|| async { "OK" }))
        .nest("/rspc", router().endpoint(|| MyCtx {}).axum())
        // .route(
        //     "/rspc/:id",
        //
        //     router()
        //         .endpoint(move || MyCtx { })
        //         .axum(),
        // )
        .fallback(|| async { "404 Not Found: We're past the event horizon..." });

    let mut addr = "[::]:8080".parse::<SocketAddr>().unwrap(); // This listens on IPv6 and IPv4

    addr.set_port(PORT);

    println!("Listening on http://localhost:{}", PORT);
    let (_tx, rx) = tokio::sync::oneshot::channel::<()>();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
        })
        .await
        .expect("Error with HTTP server!");
}
