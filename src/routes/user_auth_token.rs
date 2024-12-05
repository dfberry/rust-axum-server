use axum::{
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Path, Json, Extension, Query},
};
use std::sync::Arc;
use crate::{state::AppState};
use serde::Deserialize;
use serde_json::json;

use crate::database::database::establish_connection;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = osb_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserToken {
    id: String,
    user_id: String,
    access_token: String,
    created_at: chrono::NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = osb_token)]
pub struct NewUserToken<'a> {
    pub user_id: &'a str,
    pub access_token: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = osb_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserSession {
    id: String,
    user_id: String,
    access_token: String,
    created_at: chrono::NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = osb_session)]
pub struct NewUserSession<'a> {
    pub user_id: &'a str,
    pub access_token: &'a str,
}

pub async fn create_new_auth_token(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<NewUserToken>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    match crate::database::user_token::create_new_user_token(
        &mut connection,
        &body.user_id,
        &body.access_token,
    ).await {
        Ok(_) => {
            Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::empty())
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}
pub async fn get_auth_token(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<NewUserToken>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    match crate::database::user_token::get_user_token(
        &mut connection,
        &query.user_id,
    ).await {
        Some(token) => {
            let json_token = json!(token);
            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_token.to_string()))
                .unwrap()
        }
        None => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        }
    }
}

pub async fn delete_auth_token(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<NewUserToken>,
) -> impl IntoResponse {
    let mut connection = establish_connection();

    match crate::database::user_token::delete_user_token(
        &mut connection,
        &query.user_id,
    ).await {
        Ok(_) => {
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty())
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}