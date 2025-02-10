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
use crate::{state::AppState};

use crate::database::database::establish_connection;
use crate::database::page::{
    PagedResult,
    PaginationParams
};
use crate::database::watch::{
    create_watch,
    list_watches,
    list_watches_by_user,
    delete_user_watch,
    get_user_watch
};
use crate::utils::orgrepo::is_valid_orgrepo;

use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
pub struct NewWatchRequestBody {
    repo_name: String
}
use diesel::result::Error as DieselError;
pub fn handle_diesel_error(err: DieselError) -> Response {
    match err {
        DieselError::NotFound => {
            // Return NOT_FOUND status if the watch or resource is not found.
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        }
        _ => {
            let error_message = json!({ "error": format!("{}", err) }).to_string();
            // If error message mentions "user", return BAD_REQUEST.
            if error_message.to_lowercase().contains("user") {
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(json!({ "error": format!("{}", err) }).to_string()))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(error_message))
                    .unwrap()
            }
        }
    }
}

pub async fn get_user_watch_handler(
    Extension(_): Extension<Arc<AppState>>,
    Path(github_user_id): Path<String>,
    Path(watch_id): Path<String>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    if github_user_id.is_empty() || watch_id.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    match get_user_watch(&mut connection, &github_user_id, &watch_id).await {
        Ok(watch) => {
            let json_watch = json!(watch);
            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_watch.to_string()))
                .unwrap()
        }
        Err(err) => handle_diesel_error(err),
    }
}

pub async fn post_db_watch_new_handler(
    Path(db_user_id): Path<String>,
    Extension(_): Extension<Arc<AppState>>,
    Json(payload): Json<NewWatchRequestBody>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let repo_name = payload.repo_name.clone();

    if repo_name.is_empty() || db_user_id.is_empty() {
        let error_message = "Required fields are empty";
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(error_message))
            .unwrap();
    }

    if(!is_valid_orgrepo(&repo_name)) {
        let error_message = "Malformed repo: repo_name should be in the format org/repo or user/repo";
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(error_message))
            .unwrap();
    }

    match create_watch(&mut connection, &db_user_id, &repo_name).await {
        Ok(watch) => {
            let json_watch = json!(watch);
            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::CREATED)
                .body(Body::from(json_watch.to_string()))
                .unwrap()
        }
        Err(err) => handle_diesel_error(err),
    }
}

pub async fn get_db_watches_all_paginated_handler(
    Extension(_): Extension<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(50);

    match list_watches(&mut connection, page, page_size).await {
        Ok(PagedResult { items: watches, request_params }) => {
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
        Err(err) => handle_diesel_error(err),
    }
}

pub async fn get_db_watches_by_user_all_paginated_handler(
    Extension(_): Extension<Arc<AppState>>,
    Path(github_user_id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {

    if github_user_id.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }
    println!("github_user_id: {}", github_user_id);

    let mut connection = establish_connection();

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(50);

    match list_watches_by_user(
        &mut connection, 
        &github_user_id, 
        page, 
        page_size).await {
        Ok(PagedResult { items: watches, request_params }) => {
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
        Err(err) => handle_diesel_error(err),
    }
}

pub async fn delete_user_watch_handler(
    Extension(_): Extension<Arc<AppState>>,
    Path((github_user_id, watch_id)): Path<(String, String)>,
) -> impl IntoResponse {

    if github_user_id.is_empty() || watch_id.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    let mut connection = establish_connection();

    match get_user_watch(&mut connection, &github_user_id, &watch_id).await {
        Ok(_) => {

            println!("Delete user watch, watch exists before attempting to delete");

            match delete_user_watch(&mut connection, &github_user_id, &watch_id).await {
                Ok(_) => {
                    Response::builder()
                        .status(StatusCode::NO_CONTENT)
                        .body(Body::empty())
                        .unwrap()
                }
                Err(err) => {
                    let error_message = json!({ "error": format!("{}", err) }).to_string();
                    println!("Error: {}", error_message);
        
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(error_message))
                        .unwrap()
                }
            }
        }
        Err(err) => handle_diesel_error(err),
    }
}