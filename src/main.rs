//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    body::Body,
    extract::{Extension, Json, Path, Query},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use octocrab::models;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::sync::{Arc, RwLock};
use tokio::fs;
use toml;
use urlencoding::encode;

mod routes;
use routes::github::{
    github_get_user_handler, 
    github_post_query_issue_handler, 
    github_post_repo_handler,
};
use routes::root::root_get_handler;
use routes::user::{db_user_new_handler, db_users_all_handler, db_watch_new_handler, db_watches_all_handler, handler_config};

mod config;
use config::config::{Config, EnvFile, Package};
use config::state::AppState;

mod schema;
mod database;
mod github;

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
        .route("/", get(root_get_handler))
        .route("/github/user", get(github_get_user_handler))
        .route("/github/repo", post(github_post_repo_handler))
        .route("/github/query/issue", post(github_post_query_issue_handler))
        .route("/user", post(db_user_new_handler))
        .route("/users", get(db_users_all_handler))
        .route("/user/:username/watch", post(db_watch_new_handler))
        .route("/user/:username/watches", get(db_watches_all_handler))
        .route("/config", get(handler_config))
        .layer(Extension(shared_state.clone())); // Add the shared config to the application state;

    // run it
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
