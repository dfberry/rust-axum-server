//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    extract::{Extension},
    routing::{get, post},
    Router,
};

use std::env;
use std::sync::{Arc, RwLock};
//--------------------------------------------------
// Add the following imports for the new modules
mod state;
mod schema;
mod database;
mod github;
mod routes;

use routes::github::{
    github_get_user_handler, 
    github_post_query_issue_handler, 
    github_post_repo_handler,
    github_post_repo_stats_handler
};
use routes::root::root_get_handler;
use routes::user::{db_user_new_handler, db_users_all_handler, db_watch_new_handler, db_watches_all_handler};
use routes::admin::handler_get_config;
use state::{AppState, Config};
//--------------------------------------------------
// main
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
        .route("/github/repos/stats", post(github_post_repo_stats_handler))
        .route("/user", post(db_user_new_handler))
        .route("/users", get(db_users_all_handler))
        .route("/user/:username/watch", post(db_watch_new_handler))
        .route("/user/:username/watches", get(db_watches_all_handler))
        .route("/config", get(handler_get_config))
        .layer(Extension(shared_state.clone())); // Add the shared config to the application state;

    // run it
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
