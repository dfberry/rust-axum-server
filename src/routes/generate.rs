use axum::{
    Json,
    response::{IntoResponse, Response},
    http::{self, StatusCode},
    body::Body,
};
use crate::utils::generate::generate_unique_id;
use serde::Deserialize;
use serde_json::json;

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

    let returned_json = json!({
        "unique_id": unique_id,
        "length": length
    });

    println!("Returned JSON: {}", returned_json.to_string());

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(returned_json.to_string()))
        .unwrap()
}

