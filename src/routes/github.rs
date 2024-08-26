use axum::{
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Query, Json, Extension},
};
use std::sync::Arc;
use crate::state::AppState;
use reqwest::Client;
use serde::Deserialize;
use tokio::task;
use serde_json::json;

use crate::github::*;

#[derive(Deserialize)]
pub struct RepoRequestBody {
    token: String,
    org_or_user: String,
    repo_name: String,
}
pub async fn github_post_repo_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RepoRequestBody>,
) -> impl IntoResponse {
    let token = payload.token;
    let org_or_owner = payload.org_or_user;
    let repo_name = payload.repo_name;

    match GitHub::repo(&token, &org_or_owner, &repo_name).await {
        Ok(repo) => {
            let json_repo = json!(repo);

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_repo.to_string()))
                .unwrap()
        }
        Err(e) => {
            let error_message = format!("Error: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }
}

#[derive(Deserialize)]
pub struct QueryRequestBody {
    query: String,
}

pub async fn github_post_query_issue_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<QueryRequestBody>,
) -> impl IntoResponse {
    let token = state.config.read().unwrap().env_file.pat.clone();
    let query = payload.query; // "tokei is:pr";

    match GitHub::query(&token, &query).await {
        Ok(query_result) => {
            let json_query_result = json!(query_result);

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_query_result.to_string()))
                .unwrap()
        }
        Err(e) => {
            let error_message = format!("Error: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }
}

#[derive(Deserialize)]
pub struct UserQueryParams {
    username: String,
}
pub async fn github_get_user_handler(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<UserQueryParams>,
) -> impl IntoResponse {
    let token = state.config.read().unwrap().env_file.pat.clone();
    let username = params.username;

    match GitHub::user_profile(&token, &username).await {
        Ok(repo) => {
            let json_repo = json!(repo);

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_repo.to_string()))
                .unwrap()
        }
        Err(e) => {
            let error_message = format!("Error: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }
}
#[derive(Deserialize)]
pub struct UserQueryCustomParams {
    username: String,
    repos: Vec<String>,
}
pub async fn github_post_query_custom_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<UserQueryCustomParams>,
) -> impl IntoResponse {

    let token = state.config.read().unwrap().env_file.pat.clone();
    let repos = payload.repos;

    let stats = fetch_all_repos_stats(&token, repos).await;

    for stat in stats {
        match stat {
            Ok(repo_stats) => println!("{:?}", repo_stats),
            Err(e) => eprintln!("Error fetching repo stats: {}", e),
        }
    }
   
}   