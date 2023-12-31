#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use axum::{routing::get, Router};
use betterasb::{
    handlers::{course, courses_list, home, lunch_menus, schedule},
    scraper::parse_selectors,
    AppState,
};
use reqwest::Client;

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let client: Client = Client::builder()
        .user_agent(format!(
            "betterasb/{} (Email: tarachmel@asbarcelona.com)",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();

    let api = Router::new()
        .route("/home", get(home))
        .route("/courses/list", get(courses_list))
        .route("/courses/:student/:year/:id", get(course))
        .route("/schedule", get(schedule))
        .route("/lunch-menus", get(lunch_menus))
        .with_state(AppState { client });

    let app = Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive());

    parse_selectors();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
