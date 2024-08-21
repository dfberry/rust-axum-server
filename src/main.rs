//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    response::{
        Html,
        IntoResponse, 
        Response
    }, 
    routing::{get, post}, 
    Router, 
    http::StatusCode,
    body::Body,
    extract::{Path, Query, Json}
};
use serde::Deserialize;
use std::env;
use urlencoding::encode;
use serde_json::json;

mod config;
use config::Config;

mod github;
use github::GitHub;

#[tokio::main]
async fn main() {

    // Get the port from the environment variable
    let env_config = Config::from_env();
    let port = env_config.port.clone();
    println!("PORT: {}", port);

    let addr = format!("0.0.0.0:{}", port);
    println!("Address: {}", addr);

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/github/user", get(github_user_handler))
        .route("/github/repo", post(github_repo_handler))
        .route("/config", get(handler_config)); // Add the shared config to the application state;

    // run it
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let client_id = "Ov23liq4S3T2Ht4KUKBR";
    let redirect_uri = "http://localhost:3000/callback";
    let scope = "user";
    let encoded_redirect_uri = encode(&redirect_uri);
    let encoded_scope = scope;

    let login_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, encoded_redirect_uri, encoded_scope
    );

    let mut html_content = format!(
        "<h1><a href=\"{}\">Login</a></h1><p>",
        login_url
    );

    html_content.push_str("<h2>Environment Variables:</h2><ul>");
    for (key, value) in env::vars() {
        html_content.push_str(&format!("<li>{}: {}</li>", key, value));
    }
    html_content.push_str("</ul>");

    Html(html_content)
}

#[derive(Deserialize)]
struct UserQueryParams {
    username: String,
}
async fn github_user_handler(Query(params): Query<UserQueryParams>) -> impl IntoResponse {

    let env_config = Config::from_env();
    let token = env_config.pat.clone();
    let username = params.username;

    match GitHub::user_profile(&token,&username).await {
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
struct RepoRequestBody {
    token: String,
    org_or_user: String,
    repo_name: String,
}
async fn github_repo_handler(Json(payload): Json<RepoRequestBody>) -> impl IntoResponse {

    let env_config = Config::from_env();

    let token = payload.token;
    let org_or_owner = payload.org_or_user;
    let repo_name = payload.repo_name;

    match GitHub::repo(&token, &org_or_owner, &repo_name ).await {
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

async fn handler_config() -> Html<String> {
    let env_config = Config::from_env();

    Html(format!("Config: {:?}", env_config))
}