use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use reqwest::Response;

use crate::{
    last,
    scraper::{self, course::Course, courses_list::Courses, home::Home, schedule::Schedule},
    AppState,
};

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

// Helper function to make a get request with the session ID
async fn session_get(state: &AppState, session_id: &str, url: &str) -> Response {
    state
        .client
        .get(url)
        .header("Cookie", format!("sessionid={session_id}"))
        .send()
        .await
        .unwrap()
}

#[debug_handler]
pub async fn home(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Home>, StatusCode> {
    let resp = session_get(
        &state,
        &session_id(&headers)?,
        "https://myasb.asbarcelona.com",
    )
    .await;

    let html = resp.text().await.unwrap();
    let home = scraper::home::scrape(&html).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(home))
}

#[debug_handler]
pub async fn courses_list(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Courses>, StatusCode> {
    let resp = session_get(
        &state,
        &session_id(&headers)?,
        "https://myasb.asbarcelona.com/grading/student/my_courses",
    )
    .await;

    let year_id = last(&mut resp.url().path_segments().ok_or(StatusCode::BAD_REQUEST)?)
        .parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let html = resp.text().await.unwrap();
    let list = scraper::courses_list::scrape(&html, year_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}

#[debug_handler]
pub async fn course(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path((student, year, id)): Path<(u32, u32, u32)>,
) -> Result<Json<Course>, StatusCode> {
    let resp = session_get(
        &state,
        &session_id(&headers)?,
        &format!(
            "https://myasb.asbarcelona.com/grading/student/get_student_pws_course_grades/{}/{}/{}/",
            student, year, id
        ),
    )
    .await;

    let html = resp.text().await.unwrap();
    let course =
        scraper::course::scrape(&html, id, year).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(course))
}

#[debug_handler]
pub async fn schedule(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Schedule>, StatusCode> {
    let resp = session_get(
        &state,
        &session_id(&headers)?,
        "https://myasb.asbarcelona.com/portal/myschedule/",
    )
    .await;

    let html = resp.text().await.unwrap();
    let schedule = scraper::schedule::scrape(&html).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(schedule))
}
