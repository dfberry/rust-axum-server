use axum::{
    response::{
        Html,
        IntoResponse, 
        Response
    }, 
    routing::{get, post}, 
    Router, 
    http::{
        StatusCode,
        HeaderMap
    },
    body::Body,
    extract::{Path, Query, Json, Extension},
};
use std::sync::{Arc, RwLock};
use crate::config::state::AppState;
use urlencoding::encode;
use std::env;

use crate::database::database::establish_connection;
use crate::database::user::{
    create_user, list_users, NewUser, User
};
use crate::database::watch::{
    create_watch,
    list_watches,
    NewWatch,
    Watch,
};

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct NewUserRequestBody {
    github_user: String,
}
pub async fn db_user_new_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<NewUserRequestBody>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let github_user = payload.github_user.clone();

    let user = create_user(&mut connection, &github_user).await;

    let json_user = json!(user);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_user.to_string()))
        .unwrap()
}
pub async fn db_users_all_handler(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let mut connection = establish_connection();

    let users = list_users(&mut connection).await;

    let json_users = json!(users);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_users.to_string()))
        .unwrap()
}
#[derive(Deserialize)]
pub struct NewWatchRequestBody {
    org_repo_name: String,
    watch_type: String,
}

pub async fn db_watch_new_handler(
    Path(github_user_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<NewWatchRequestBody>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let org_repo_name = payload.org_repo_name.clone();
    let watch_type = payload.watch_type.clone();

    let watch = create_watch(
        &mut connection,
        &github_user_id,
        &org_repo_name,
        &watch_type,
    )
    .await;

    let json_watch = json!(watch);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_watch.to_string()))
        .unwrap()
}

pub async fn db_watches_all_handler(
    Path(github_user_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    let watches = list_watches(&mut connection).await;

    let json_watches = json!(watches);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_watches.to_string()))
        .unwrap()
}
pub async fn handler_config(Extension(state): Extension<Arc<AppState>>) -> Html<String> {
    // Collect environment variables
    let env_vars: String = env::vars()
        .map(|(key, value)| format!("<li>{}: {}</li>", key, value))
        .collect::<Vec<String>>()
        .join("");

    // Collect app state
    let app_state = state.config.read().unwrap();
    let app_state_html = format!(
        "<h2>App State:</h2>
        <ul>
            <li>Version: {}</li>
            <li>GitHub Client ID: {}</li>
            <li>GitHub Client Redir: {}</li>
        </ul>",
        app_state.package.version,
        app_state.env_file.github_client_id,
        app_state.env_file.github_redirect_uri
    );

    // Combine all information into HTML content
    let html_content = format!(
        "{app_state_html}
        <h2>Environment Variables:</h2>
        <ul>{env_vars}</ul>",
        app_state_html = app_state_html,
        env_vars = env_vars
    );

    Html(html_content)
}
