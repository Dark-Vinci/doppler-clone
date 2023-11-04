use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn page404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "PAGE NOT FOUND, YOU LOOK LOST")
}