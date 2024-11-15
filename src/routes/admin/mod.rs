use std::sync::Arc;
use crate::state::AppState;
use axum::{
    extract::Extension,
    response::IntoResponse,
    http::header::HeaderMap,
    Json,
};
use serde_json::json;
use std::env;
use chrono::Utc;
use crate::io::write_json_to_file;

pub async fn handler_get_config(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
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

    // let file_name = format!("app_state.json");
    // let file_path = format!("./data/{}", file_name);
    // let _ = write_json_to_file(&file_path, &returned_json).await.unwrap();


    // Create a HeaderMap and insert the custom header
    let mut headers = HeaderMap::new();
    headers.insert("x-source-board-version-config", app_state.package.version.parse().unwrap());

    // Combine the JSON response with the headers
    (headers, Json(returned_json)).into_response()
}
