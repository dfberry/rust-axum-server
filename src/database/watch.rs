use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::osb_user_custom_config;
use crate::database::page::{PagedResult, PageRequest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = osb_user_custom_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Watch {
    pub id: String,
    pub user_id: String,
    pub repo_name: String,
    pub created_at: chrono::NaiveDateTime
}


#[derive(Insertable)]
#[diesel(table_name = osb_user_custom_config)]
pub struct NewWatch<'a> {
    pub user_id: &'a str,
    pub repo_name: &'a str
}

pub async fn list_watches(
    connection: &mut PgConnection,
    page: i64,
    page_size: i64
) -> PagedResult<Watch> {

    use crate::schema::osb_user_custom_config::dsl::*;

    let offset = (page - 1) * page_size;
    let limit = page_size + 1; // Fetch one more item to check if there are more pages

    let results = osb_user_custom_config
        .offset(offset)
        .limit(limit)
        .select(Watch::as_select())
        .load(connection)
        .expect("Error loading watches");

    println!("Displaying {} watches", results.len());

    let has_more = results.len() as i64 > page_size;
    let items = if has_more {
        results.into_iter().take(page_size as usize).collect()
    } else {
        results
    };

    let page_request_params = PageRequest {
        page,
        page_size,
        has_more
    };


    println!("Displaying {} watches", items.len());

    PagedResult { items, request_params: page_request_params }
}

pub async fn create_watch(connection: &mut PgConnection, user_id: &str, repo_name: &str) -> Watch {

    use crate::schema::osb_user_custom_config;

    let new_watch = NewWatch { 
        user_id: &user_id, 
        repo_name: &repo_name
    };

    diesel::insert_into(osb_user_custom_config::table)
        .values(&new_watch)
        .returning(Watch::as_returning())
        .get_result(connection)
        .expect("Error saving new watch")
}


pub async fn list_watches_by_user(
    connection: &mut PgConnection,
    db_user_id: &str,
    page: i64,
    page_size: i64,
) -> PagedResult<Watch> {
    use crate::schema::osb_user_custom_config::dsl::*;

    println!("Listing watches for user: {}", db_user_id);

    let offset = (page - 1) * page_size;
    let limit = page_size + 1; // Fetch one more item to check if there are more pages


    let results: Vec<Watch> = osb_user_custom_config
        .filter(user_id.eq(db_user_id))
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

    let page_request_params = PageRequest {
        page,
        page_size,
        has_more
    };


    println!("Displaying {} watches", items.len());

    PagedResult { items, request_params: page_request_params }
}