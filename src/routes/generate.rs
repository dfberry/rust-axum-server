use serde::Deserialize;
use axum::Json;
use crate::utils::generate::generate_unique_id;
use axum::response::{IntoResponse, Response};
use axum::http::{self, StatusCode};
use axum::body::Body;

#[derive(Deserialize)]
pub struct GenerateUniqueIdRequestBody {
    length: Option<usize>,
}
pub async fn handler_generate_unique_id(
    Json(payload): Json<GenerateUniqueIdRequestBody>,
) -> impl IntoResponse {

    let length = match payload.length {
        Some(ref length) => length.clone(),
        None => 15, // Default length
    };

    let unique_id: String = generate_unique_id(length);
    println!("Generated unique id: {}", unique_id);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(unique_id))
        .unwrap()
}

