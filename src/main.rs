use axum::{
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn summon_handler(headers: HeaderMap) -> impl IntoResponse {
    let user_agent_value = headers.get("User-Agent").unwrap();
    let user_agent = user_agent_value.to_str().unwrap();

    if user_agent.contains("curl") {
        let content = include_str!("lib/about.md");
        content.into_response()
    } else {
        Redirect::temporary("https://sideral.design").into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(summon_handler));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
