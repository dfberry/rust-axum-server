//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post, delete},
    Router,
};
use axum::Extension;
use hyper::header::HeaderValue;
//use std::env;
use std::sync::{Arc, RwLock};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//--------------------------------------------------
// Add the following imports for the new modules
mod database;
mod github;
mod routes;
mod schema;
mod state;
mod utils;

use state::get_cargo_version;

use routes::admin::handler_get_config;
use routes::github::{
    github_get_query_handler, github_get_repo_issues_handler, github_get_repo_prs_handler,
    github_get_user_by_token, github_get_user_handler, github_get_user_profile_handler,
    github_get_user_rate_limit_handler, github_post_query_issue_handler, github_post_repo_handler,
    github_post_repo_stats_handler,
};
use routes::root::root_get_handler;
use routes::user::{
    post_db_user_new_handler, 
    get_db_users_all_paginated_handler,
    get_db_user_get_handler
};
use routes::generate::handler_generate_unique_id;
use routes::user_watch::{
    post_db_watch_new_handler,  
    get_db_watches_by_user_all_paginated_handler,
    get_db_watches_all_paginated_handler,
    delete_user_watch_handler,
    get_user_watch_handler
};
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

    // build our application with a route
    let app = Router::new()
        .route("/", get(root_get_handler))
        .route("/github/query/issue", post(github_post_query_issue_handler))
        .route("/github/query", get(github_get_query_handler)) 
        .route("/github/repos/stats", post(github_post_repo_stats_handler))
        .route("/github/repo/issues", get(github_get_repo_issues_handler)) 
        .route("/github/repo/prs", get(github_get_repo_prs_handler)) 
        .route("/github/repo", post(github_post_repo_handler))
        .route("/github/user/token", get(github_get_user_by_token))
        .route("/github/user/rate-limit",get(github_get_user_rate_limit_handler)) 
        .route("/github/user/:username",get(github_get_user_profile_handler))
        .route("/github/user", post(github_get_user_handler))

        .route("/users/watches", get(get_db_watches_all_paginated_handler))
        .route("/users", get(get_db_users_all_paginated_handler))

        .route("/user/:username/watches", get(get_db_watches_by_user_all_paginated_handler))
        .route("/user/:username/watch/:watch_id", get(get_user_watch_handler))
        .route("/user/:username/watch/:watch_id", delete(delete_user_watch_handler))
        .route("/user/:username/watch", post(post_db_watch_new_handler))
        .route("/user/:username", get(get_db_user_get_handler))
        .route("/user", post(post_db_user_new_handler))

        .route("/config", get(handler_get_config))
        .route("/generate/uniqueid", post(handler_generate_unique_id))
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
async fn response_version_header(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let version = get_cargo_version().await.unwrap();

    // do something with `response`...
    response.headers_mut().insert(
        "x-source-board-version",
        HeaderValue::from_str(&version).unwrap(),
    );

    response
}
