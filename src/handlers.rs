use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    last,
    scraper::{self, Course, CourseListitem, Home},
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

#[debug_handler]
pub async fn home(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Home>, StatusCode> {
    let session_id = session_id(&headers)?;

    let resp = state
        .client
        .get("https://myasb.asbarcelona.com")
        .header("Cookie", format!("sessionid={session_id}"))
        .send()
        .await
        .unwrap();

    let html = resp.text().await.unwrap();
    let home = scraper::home(&html);

    Ok(Json(home))
}

#[derive(Serialize, Deserialize)]
pub struct Courses {
    pub year_id: u16,
    pub courses: Box<[CourseListitem]>,
}

#[debug_handler]
pub async fn courses_list(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Courses>, StatusCode> {
    let session_id = session_id(&headers)?;

    let resp = state
        .client
        .get("https://myasb.asbarcelona.com/grading/student/my_courses")
        .header("Cookie", format!("sessionid={session_id}"))
        .send()
        .await
        .unwrap();

    let year_id = last(&mut resp.url().path_segments().unwrap())
        .parse()
        .unwrap();

    let html = resp.text().await.unwrap();
    let list = scraper::courses_list(&html);

    Ok(Json(Courses {
        year_id,
        courses: list,
    }))
}

#[debug_handler]
pub async fn course(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path((student, year, id)): Path<(u32, u32, u32)>,
) -> Result<Json<Course>, StatusCode> {
    let session_id = session_id(&headers)?;

    let resp = state
        .client
        .get(format!(
            "https://myasb.asbarcelona.com/grading/student/get_student_pws_course_grades/{}/{}/{}/",
            student, year, id
        ))
        .header("Cookie", format!("sessionid={session_id}"))
        .send()
        .await
        .unwrap();

    let html = resp.text().await.unwrap();
    let course = scraper::course(&html, id, year);

    Ok(Json(course))
}
