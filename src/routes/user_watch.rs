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
use crate::database::page::{
    PagedResult,
    PaginationParams
};
use crate::database::watch::{
    create_watch,
    list_watches,
    list_watches_by_user
};

use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
pub struct NewWatchRequestBody {
    user_id: String,
    repo_name: String
}

pub async fn post_db_watch_new_handler(
    //Path(github_user_id): Path<String>,
    Extension(_): Extension<Arc<AppState>>,
    Json(payload): Json<NewWatchRequestBody>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let repo_name = payload.repo_name.clone();
    let user_id = payload.user_id.clone();

    if repo_name.is_empty() || user_id.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    let watch = create_watch(
        &mut connection,
        &user_id,
        &repo_name
    )
    .await;

    let json_watch = json!(watch);

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_watch.to_string()))
        .unwrap()
}

pub async fn get_db_watches_all_paginated_handler(
    Extension(_): Extension<Arc<AppState>>,
    Json(payload): Json<PaginationParams>
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let page = payload.page.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(50);

    let PagedResult { items: watches, request_params } = list_watches(&mut connection, page, page_size).await;

    let json_response = json!({
        "watches": watches,
        "request_params": request_params,
    });

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_response.to_string()))
        .unwrap()
}

pub async fn get_db_watches_by_user_all_paginated_handler(
    Extension(_): Extension<Arc<AppState>>,
    Path(github_user_id): Path<String>,
    Json(payload): Json<PaginationParams>,
) -> impl IntoResponse {

    if github_user_id.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }
    println!("github_user_id: {}", github_user_id);

    let mut connection = establish_connection();

    let page = payload.page.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(50);

    let PagedResult { items: watches, request_params } = list_watches_by_user(&mut connection, &github_user_id, page, page_size).await;

    let json_response = json!({
        "watches": watches,
        "request_params": request_params,
    });

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_response.to_string()))
        .unwrap()
}