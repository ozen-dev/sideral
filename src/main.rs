pub mod helpers;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(helpers::summon));
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
