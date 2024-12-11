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
use mongodb::{error::Result, Client, Collection, Database};
use std::sync::Arc;
use crate::{database, mongo_database::log::{get_connection, read_log}};
use crate::state::AppState;
use mongodb::bson::{doc, Document};
use serde::Deserialize;
use serde_json::json;
use crate::mongo_database::models::FlattenedRepoData;
use futures::TryStreamExt;

#[derive(Deserialize)]
pub struct LogQuery {
    org: String,
    repo: String,
    // begin_date: String,
    // end_date: String,
}

pub async fn get_db_mongo_log_handler(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<LogQuery>,
) -> impl IntoResponse {

    let org = &params.org;
    let repo = &params.repo;
    // let begin_date = &params.begin_date;
    // let end_date = &params.end_date;

    let filter = doc! {
        "org": org,
        "repo": repo
    };

    let collection_client: Collection<FlattenedRepoData> = state.config.read().unwrap().mongo.collection_client.clone();

    match collection_client.find(filter).await {
        Ok(mut cursor) => {
            let mut results: Vec<FlattenedRepoData> = Vec::new();
            while let Some(doc) = cursor.try_next().await.unwrap() {
                results.push(doc);
            }

            let returned_json = json!(results);
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