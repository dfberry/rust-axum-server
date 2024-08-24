use axum::{
    response::Html, 
    extract::Extension,
};
use std::sync::Arc;
use crate::state::AppState;

use std::env;

pub async fn handler_get_config(Extension(state): Extension<Arc<AppState>>) -> Html<String> {
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
