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
    http::{
        StatusCode,
        HeaderMap
    },
    body::Body,
    extract::{Path, Query, Json, Extension},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use std::env;
use std::sync::{Arc, RwLock};
use urlencoding::encode;
use serde_json::json;
use tokio::fs;
use toml;


mod config;
use config::{
    Config,
    Package,
    EnvFile
};


mod github;
use github::GitHub;

#[derive(Clone)]
pub struct AppState {
    config: Arc<RwLock<Config>>,
}

#[tokio::main]
async fn main() {

    // Get the port from the environment variable
    let env_config = match Config::get().await {
        Ok(env_config) => env_config,
        Err(e) => {
            eprintln!("Failed to get config: {}", e);
            std::process::exit(1);
        }
    };

    // Construct AppState with Arc and RwLock
    let app_state = AppState {
        config: Arc::new(RwLock::new(env_config)),
    };

    // Wrap the AppState in an Arc to share it across handlers
    let shared_state = {
        let app_state = app_state.clone();
        Arc::new(app_state)
    };
    
    let port = &shared_state.config.read().unwrap().env_file.port;
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
        .route("/config", get(handler_config))
        .layer(Extension(shared_state.clone())); // Add the shared config to the application state;

    // run it
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Extension(state): Extension<Arc<AppState>>) -> Html<String> {
    let client_id = "Ov23liq4S3T2Ht4KUKBR";
    let redirect_uri = "http://localhost:3000/callback";
    let scope = "user";
    let encoded_redirect_uri = encode(&redirect_uri);
    let encoded_scope = scope;

    let login_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, encoded_redirect_uri, encoded_scope
    );



        let html_content = format!(
            "<h1><a href=\"{login_url}\">Login</a></h1>"
        );

    Html(html_content)
}

#[derive(Deserialize)]
struct UserQueryParams {
    username: String,
}
async fn github_user_handler(Extension(state): Extension<Arc<AppState>>, Query(params): Query<UserQueryParams>) -> impl IntoResponse {


    let token = state.config.read().unwrap().env_file.pat.clone();
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
async fn github_repo_handler(Extension(state): Extension<Arc<AppState>>, Json(payload): Json<RepoRequestBody>) -> impl IntoResponse {

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

async fn handler_config(
    Extension(state): Extension<Arc<AppState>>
) -> Html<String> {
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