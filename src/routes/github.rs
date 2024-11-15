use axum::{
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Path, Query, Json, Extension},
};
use std::sync::Arc;
use crate::state::AppState;
use serde::Deserialize;
use serde_json::json;
use crate::github::GitHub;
use crate::github::GitHubApi;
use crate::github::fetch_all_repos_stats;
use crate::io::write_json_to_file;

#[derive(Deserialize)]
pub struct RepoRequestBody {
    token: Option<String>,
    org_or_user: String,
    repo_name: String,
}
pub async fn github_post_repo_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RepoRequestBody>,
) -> impl IntoResponse {

    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };

    let org_or_owner = payload.org_or_user;
    let repo_name = payload.repo_name;

    // if the token is empty, return a 401 Unauthorized
    if token.is_empty() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap();
    }

    if (org_or_owner.is_empty() || repo_name.is_empty()) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    match GitHub::repo(&token, &org_or_owner, &repo_name).await {
        Ok(repo) => {
            let json_repo = json!(repo);

            let file_name = format!("github_repo_{}_{}.json", org_or_owner, repo_name);
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_repo).await.unwrap();

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
    token: Option<String>,
}

pub async fn github_post_query_issue_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<QueryRequestBody>,
) -> impl IntoResponse {

    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };

    let query = payload.query; // "tokei is:pr";

    if (query.is_empty()) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    match GitHub::query(&token, &query).await {
        Ok(query_result) => {
            let json_query_result = json!(query_result);

            let file_name = format!("github_query.json");
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_query_result).await.unwrap();


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
    username: String
}

#[derive(Deserialize)]
pub struct UserQueryRequestBody {
    token: Option<String>,
}
pub async fn github_get_user_handler(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<UserQueryParams>,
    Json(payload): Json<UserQueryRequestBody>,
) -> impl IntoResponse {

    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };

    let username = params.username;

    match GitHub::user_profile(&token, &username).await {
        Ok(repo) => {
            let json_repo = json!(repo);

            let file_name = format!("github_user.json");
            let file_path = "./data/{}";
            let _ = write_json_to_file(&file_path, &file_name, &json_repo).await.unwrap();


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
pub struct RepoStatsParams {
    repos: Vec<String>,
    token: Option<String>,
}

pub async fn github_post_repo_stats_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RepoStatsParams>,
) -> impl IntoResponse {

    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };
    let repos = payload.repos;

    let stats = fetch_all_repos_stats(&token, repos).await;
    let json_stats = json!(stats);

    let file_name = format!("github_repos_stats.json");
    let file_path = "./data/";
    let _ = write_json_to_file(&file_path, &file_name, &json_stats).await.unwrap();



    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_stats.to_string()))
        .unwrap()
}   

#[derive(Deserialize)]
pub struct GetProfileRequest {
    token: Option<String>,
}

pub async fn github_get_user_profile_handler(
    Path(username): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<GetProfileRequest>,
) -> impl IntoResponse {
    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };
    let username = username;

    match GitHub::user_profile(&token, &username).await {
        Ok(repo) => {
            let json_repo = json!(repo);

            let file_name = format!("github_user_profile_{}.json", username);
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_repo).await.unwrap();

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
pub struct RepoIssuesRequestBody {
    token: Option<String>,
    org_or_user: String,
    repo_name: String,
}
pub async fn github_get_repo_issues_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RepoRequestBody>,
) -> impl IntoResponse {
    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };
    let org_or_owner = payload.org_or_user;
    let repo_name = payload.repo_name;

    // if the token is empty, return a 401 Unauthorized
    if token.is_empty() {

        println!("github_post_repo_handler::Token is empty");

        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap();
    }

    if (org_or_owner.is_empty() || repo_name.is_empty()) {

        println!("github_post_repo_handler::Org or repo name is empty");

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    println!("github_post_repo_handler::Fetching org_or_owner & repo: {} and {}", org_or_owner, repo_name);
    println!("github_post_repo_handler::Token: {}", token);

    match GitHub::repo_issues(&token, &org_or_owner, &repo_name).await {
        Ok(repo) => {
            println!("github_post_repo_handler::Fetched repo ok: {}/{}", org_or_owner, repo_name);

            let json_repo = json!(repo);

            let file_name = format!("github_repo_issues_{}_{}.json", org_or_owner, repo_name);
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_repo).await.unwrap();
        
        

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_repo.to_string()))
                .unwrap()
        }
        Err(e) => {
            println!("github_post_repo_handler::Fetched repo error: {}/{}", org_or_owner, repo_name);

            let error_message = format!("Error: {:?}", e);
            println!("github_post_repo_handler::Error: {:?}", error_message);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }
}

pub async fn github_get_repo_prs_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RepoRequestBody>,
) -> impl IntoResponse {
    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };
    let org_or_owner = payload.org_or_user;
    let repo_name = payload.repo_name;

    // if the token is empty, return a 401 Unauthorized
    if token.is_empty() {

        println!("github_post_repo_handler::Token is empty");

        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap();
    }

    if (org_or_owner.is_empty() || repo_name.is_empty()) {

        println!("github_post_repo_handler::Org or repo name is empty");

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    println!("github_post_repo_handler::Fetching org_or_owner & repo: {} and {}", org_or_owner, repo_name);
    println!("github_post_repo_handler::Token: {}", token);

    match GitHub::repo_prs(&token, &org_or_owner, &repo_name).await {
        Ok(repo) => {
            println!("github_post_repo_handler::Fetched repo ok: {}/{}", org_or_owner, repo_name);

            let json_repo = json!(repo);

            let file_name = format!("github_repo_prs_{}_{}.json", org_or_owner, repo_name);
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_repo).await.unwrap();
        

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_repo.to_string()))
                .unwrap()
        }
        Err(e) => {
            println!("github_post_repo_handler::Fetched repo error: {}/{}", org_or_owner, repo_name);

            let error_message = format!("Error: {:?}", e);
            println!("github_post_repo_handler::Error: {:?}", error_message);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }
}

pub async fn github_get_query_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<QueryRequestBody>,
) -> impl IntoResponse {
    // if the query token is empty, use the state token
    let token = match payload.token {
        Some(t) => t,
        None => state.config.read().unwrap().env_file.pat.clone(),
    };
    let query = payload.query; // "tokei is:pr";

    if (query.is_empty()) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    match GitHub::query(&token, &query).await {
        Ok(query_result) => {
            let json_query_result = json!(query_result);

            let file_name = format!("github_get_query.json");
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_query_result).await.unwrap();

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
pub struct GitHubRateLimitRequestBody {
    token: String,
    username: String,
}
pub async fn github_get_user_rate_limit_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<GitHubRateLimitRequestBody>,
) -> impl IntoResponse {
    let token = payload.token;
    let username = payload.username;

    if(token.is_empty()) {
        return Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .body(Body::empty())
            .unwrap();
    }

    match GitHubApi::rate_limit(&token).await {
        Ok(query_result) => {
            let json_query_result = json!(&query_result);

            let file_name = format!("github_user_rate_limit_{}.json", &username);
            let file_path = "./data/";
            let _ = write_json_to_file(&file_path, &file_name, &json_query_result).await.unwrap();

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