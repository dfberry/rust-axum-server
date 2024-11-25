use diesel::pg::PgConnection;
use diesel::prelude::*;
use http::request;
use crate::schema::watches;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = watches)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Watch {
    pub id: String,
    pub github_user_id: String,
    pub org_repo_name: String,
    pub watch_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = watches)]
pub struct NewWatch<'a> {
    pub github_user_id: &'a str,
    pub org_repo_name: &'a str,
    pub watch_type: &'a str,
}

pub async fn list_watches(connection: &mut PgConnection) -> Vec<Watch> {

    use crate::schema::watches::dsl::*;

    let results = watches
        .limit(5)
        .select(Watch::as_select())
        .load(connection)
        .expect("Error loading watches");

    println!("Displaying {} watches", results.len());

    results
}

pub async fn create_watch(connection: &mut PgConnection, github_user_id: &str, org_repo_name: &str, watch_type: &str) -> Watch {

    use crate::schema::watches;

    let new_watch = NewWatch { 
        github_user_id: &github_user_id, 
        org_repo_name: &org_repo_name, 
        watch_type: &watch_type 
    };

    diesel::insert_into(watches::table)
        .values(&new_watch)
        .returning(Watch::as_returning())
        .get_result(connection)
        .expect("Error saving new watch")
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserWatchRequest {
    page: i64,
    page_size: i64,
    pub has_more: bool,
}   
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub request_params: UserWatchRequest,   
}

pub async fn list_watches_by_user(
    connection: &mut PgConnection,
    db_github_user_id: &str,
    page: i64,
    page_size: i64,
) -> PagedResult<Watch> {
    use crate::schema::watches::dsl::*;

    println!("Listing watches for user: {}", db_github_user_id);

    let offset = (page - 1) * page_size;
    let limit = page_size + 1; // Fetch one more item to check if there are more pages


    let results: Vec<Watch> = watches
        .filter(github_user_id.eq(db_github_user_id))
        .offset(offset)
        .limit(limit)
        .select(Watch::as_select())
        .load(connection)
        .expect("Error loading watches");

    let has_more = results.len() as i64 > page_size;
    let items = if has_more {
        results.into_iter().take(page_size as usize).collect()
    } else {
        results
    };

    let page_request_params = UserWatchRequest {
        page,
        page_size,
        has_more
    };


    println!("Displaying {} watches", items.len());

    PagedResult { items, request_params: page_request_params }
}