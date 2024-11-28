use crate::database::page::{PagedResult, PageRequest};

use crate::schema::osb_user;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = osb_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub github_id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = osb_user)]
pub struct NewUser<'a> {
    pub github_id: &'a str,
    pub username: &'a str,
}
pub async fn get_user(
    connection: &mut PgConnection,
    db_user_id: &str,
) -> Option<User> {
    use crate::schema::osb_user::dsl::*;

    println!("get_user User: {:?}", db_user_id);

    let users = osb_user
    .filter(id.eq(db_user_id))
        .limit(1)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

        // consume the Vec<User> and return the first item directly
        if let Some(user) = users.into_iter().next() {
            println!("Display user: {:?}", user);
            Some(user)
        } else {
            println!("No user found with github_user_id: {}", db_user_id);
            None
        }
}
pub async fn list_users(
    connection: &mut PgConnection,
    page: i64,
    page_size: i64,
) -> PagedResult<User> {
    use crate::schema::osb_user::dsl::*;

    let offset = (page - 1) * page_size;
    let limit = page_size + 1; // Fetch one more item to check if there are more pages

    let results = osb_user
        .offset(offset)
        .limit(limit)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

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
pub async fn create_user(connection: &mut PgConnection, github_id: &str, username: &str) -> User {
    use crate::schema::osb_user;

    let new_user = NewUser {
        github_id: &github_id,
        username: &username,
    };

    diesel::insert_into(osb_user::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")
}
