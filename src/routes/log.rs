use axum::{
    debug_handler,
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Path, Json, Extension, Query},
};
use mongodb::{error::Result, Client, Collection, Database, options::FindOptions};
use std::{ptr::null, sync::Arc};
use crate::state::AppState;
use mongodb::bson::{doc, Document};
use serde::Deserialize;
use serde_json::json;
use crate::mongo_database::{
    models::FlattenedRepoData,
    crud::{
        get_connection,
        read,
    }
};
use futures::TryStreamExt;

#[derive(Deserialize)]
pub struct LogQuery {
    org: String,
    repo: String,
    limit: Option<i64>,
}

pub async fn get_db_mongo_log_handler(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<LogQuery>,
) -> impl IntoResponse {

    let org = &params.org;
    let repo = &params.repo;
    let limit = params.limit.unwrap_or(30);

    let filter = doc! {
        "org": org,
        "repo": repo
    };

    let sort: Document = doc! {
        "log_time": -1 //descending
    };

    let collection_client = state.config.read().unwrap().mongo.collection_client.clone();

    match read(&collection_client , filter, sort, limit).await {
        Ok(mut items) => {

            let returned_json = json!(items);
            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(returned_json.to_string()))
                .unwrap()
        }
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": format!("Failed to read log: {}", e)
            });
            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_response.to_string()))
                .unwrap()
        }
    }

}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}