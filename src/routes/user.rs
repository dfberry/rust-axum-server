use axum::{
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Path, Json, Extension},
};
use std::sync::Arc;
use crate::state::AppState;

use crate::database::database::establish_connection;
use crate::database::user::{
    create_user, list_users
};
use crate::database::watch::{
    create_watch,
    list_watches
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