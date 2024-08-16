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
    routing::get, 
    Router, 
    http::StatusCode,
    body::Body
};

use std::env;
use urlencoding::encode;
use octocrab::Octocrab;
// use serde::Serialize;
use serde_json::json;

mod config;
use config::Config;

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
// #[derive(Serialize)]
// struct GitHubUserResponse {
//     status: String,
// }
// #[derive(Deserialize)]
// struct GitHubUser {
//     login: String,
//     id: u64,
//     // Add other fields as needed
// }
async fn github_user_handler() -> impl IntoResponse {
    let env_config = Config::from_env();
    let token = env_config.pat.clone();
    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    match octocrab
    .repos("dfberry", "azure-notes")
    .get()
    .await
{
    Ok(repo) => {
        let json_repo = json!(repo);

        Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .status(StatusCode::OK)
            .body(Body::from(json_repo.to_string()))
            .unwrap()
    }
    Err(e) => {
        let error_message = json!({ "error": e.to_string() });
        let error_body = Body::from(error_message.to_string());

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(error_body)
            .unwrap()
    }
}
}

async fn handler_config() -> Html<String> {
    let env_config = Config::from_env();

    Html(format!("Config: {:?}", env_config))
}