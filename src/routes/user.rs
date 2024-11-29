use axum::{
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Path, Json, Extension, Query},
};
use std::sync::Arc;
use crate::state::AppState;

use crate::database::database::establish_connection;
use crate::database::user::{
    create_user, list_users, get_user
};
use crate::database::page::{
    PagedResult,
    PaginationParams
};

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct NewUserRequestBody {
    github_id: String,
    username: String
}
pub async fn post_db_user_new_handler(
    Path(github_user): Path<String>,
    Extension(_): Extension<Arc<AppState>>,
    Json(payload): Json<NewUserRequestBody>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let github_id = payload.github_id.clone();
    let username = payload.username.clone();

    if github_user.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    let user = create_user(&mut connection, &github_id, &username).await;

    let json_user = json!(user);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_user.to_string()))
        .unwrap()
}

pub async fn get_db_user_get_handler(
    Path(github_user_id): Path<String>,
    Extension(_): Extension<Arc<AppState>>
) -> impl IntoResponse {
    let mut connection = establish_connection();
    println!("get_db_user_get_handler User: {:?}", github_user_id);

    let user = get_user(&mut connection, &github_user_id).await;

    let json_user = json!(user);
    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_user.to_string()))
        .unwrap()
}

pub async fn get_db_users_all_paginated_handler(
    Extension(_): Extension<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(50);

    let PagedResult { items: users, request_params } = list_users(&mut connection, page, page_size).await;

    let json_response = json!({
        "users": users,
        "request_params": request_params,
    });

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_response.to_string()))
        .unwrap()
}