//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    extract::{Extension},
    body::{Body, Bytes},
    routing::{get, post},
    Router,
    extract::Request,
    http::{response, StatusCode},
    response::{IntoResponse, Response},
    middleware::{self, Next},
};
use http_serde_ext::header_value;
use hyper::header::HeaderValue;
use http_body_util::BodyExt;
use std::env;
use std::sync::{Arc, RwLock};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower::ServiceBuilder;
//--------------------------------------------------
// Add the following imports for the new modules
mod state;
mod schema;
mod database;
mod github;
mod routes;
mod utils;

use state::get_cargo_version;

use routes::github::{
    github_get_query_handler, github_get_repo_issues_handler, github_get_repo_prs_handler, github_get_user_by_token, github_get_user_handler, github_get_user_profile_handler, github_get_user_rate_limit_handler, github_post_query_issue_handler, github_post_repo_handler, github_post_repo_stats_handler
};
use github::GitHubApi;
use routes::root::root_get_handler;
use routes::user::{db_user_new_handler, db_users_all_handler, db_watch_new_handler, db_watches_all_handler, db_watches_by_user_all_handler};
use routes::admin::handler_get_config;
use state::{AppState, Config};
//--------------------------------------------------
// main
#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

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

        .route("/github/user/token", get(github_get_user_by_token))
        .route("/github/user/rate-limit", get(github_get_user_rate_limit_handler)) // 
        .route("/github/user/:username", get(github_get_user_profile_handler))   // NEW
        .route("/github/repo/issues", get(github_get_repo_issues_handler))       // NEW
        .route("/github/repo/prs", get(github_get_repo_prs_handler))             // NEW
        .route("/github/query", get(github_get_query_handler))                   // NEW
        .route("/github/user", post(github_get_user_handler))
        .route("/github/repo", post(github_post_repo_handler))
        .route("/github/query/issue", post(github_post_query_issue_handler))
        .route("/github/repos/stats", post(github_post_repo_stats_handler))
        .route("/user", post(db_user_new_handler))
        .route("/users", get(db_users_all_handler))
        .route("/user/:username/watch", post(db_watch_new_handler))
        .route("/user/:username/watches", get(db_watches_all_handler)) // TO BE RETIRED
        .route("/user/:db_github_user_id/watches/list", get(db_watches_by_user_all_handler)) // REPLACED WITH THIS
        .route("/config", get(handler_get_config))
        .layer(Extension(shared_state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .layer(middleware::from_fn(response_version_header));

    // run it
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
async fn response_version_header(
    request: Request, 
    next: Next
) -> Response {
    let mut response = next.run(request).await;
    let version = get_cargo_version().await.unwrap();

    // do something with `response`...
    response.headers_mut().insert("x-source-board-version", HeaderValue::from_str(&version).unwrap());

    response
}
