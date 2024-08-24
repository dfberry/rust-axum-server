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
use std::sync::{Arc, RwLock};
use crate::config::state::AppState;
use urlencoding::encode;
pub async fn root_get_handler(Extension(state): Extension<Arc<AppState>>) -> Html<String> {
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