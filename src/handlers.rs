use axum::{
    debug_handler,
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::scraper;
use crate::AppState;

// Helper function to get the session ID from the header map and return HTTP 400 if it doesn't exist.
fn session_id(headers: &HeaderMap) -> Result<&str, StatusCode> {
    let Some(session_id) = headers.get("SessionID") else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let Ok(session_id) = session_id.to_str() else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(session_id)
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub text: String,
}

#[debug_handler]
pub async fn day(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Day>, StatusCode> {
    let session_id = session_id(&headers)?;

    let resp = state
        .client
        .get("https://myasb.asbarcelona.com")
        .header("Cookie", format!("sessionid={session_id}"))
        .send()
        .await
        .unwrap();

    let html = resp.text().await.unwrap();
    let text = scraper::day(&html);

    Ok(Json(Day { text }))
}
