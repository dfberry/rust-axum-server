use std::sync::Arc;
use crate::state::AppState;
use axum::{
    extract::Extension,
    extract::Query,
    response::{IntoResponse, Response},
    http::header::HeaderMap,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::env;
use chrono::Utc;

#[derive(Deserialize)]
pub struct AdminQuery {
    pub admin_key: Option<String>,
}

pub async fn handler_get_config(
    Query(params): Query<AdminQuery>,
    Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {

    let admin_key = match params.admin_key {
        Some(key) => {
            println!("Received admin_key: {}", key);
            key
        }
        None => {
            println!("Missing admin_key");
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Missing key".into())
                .unwrap();
        }
    };
    println!("Query string admin key: {}", admin_key);

    // Get the ADMIN_KEY from the environment variables
    let env_admin_key = env::var("ADMIN_KEY").unwrap_or_default();
    println!("Environment variable admin key: {}", env_admin_key);

    // Check if the provided ADMIN_KEY matches the environment variable
    if admin_key.to_lowercase() != env_admin_key.to_lowercase() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid key".into())
            .unwrap();
    }

    // Collect environment variables
    let env_vars = env::vars()
        .map(|(key, value)| json!({ "key": key, "value": value }))
        .collect::<Vec<_>>();

    // Collect app state
    let app_state = state.config.read().unwrap();

    // Get the current UTC date-time
    let current_time = Utc::now().to_rfc3339();

    // Create JSON response
    let returned_json = json!({
        "env_vars": env_vars,
        "app_state": {
            "version": app_state.package.version,
            "github_redirect_uri": app_state.env_file.github_redirect_uri,
        },
        "timestamp": current_time
    });

    // Create a HeaderMap and insert the custom header
    let mut headers = HeaderMap::new();
    headers.insert("x-source-board-version-config", app_state.package.version.parse().unwrap());

    // Combine the JSON response with the headers
    (headers, Json(returned_json)).into_response()
}
