use axum::{routing::get, Router};
use myasb::AppState;
use reqwest::Client;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let client: Client = Client::builder()
        .user_agent(format!(
            "better-myasb/{} (Email: tarachmel@asbarcelona.com)",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();

    let api = Router::new()
        .route("/test", get(myasb::handlers::day))
        .with_state(AppState { client });

    let app = Router::new().nest("/api", api);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
