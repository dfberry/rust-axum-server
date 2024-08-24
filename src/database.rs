use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Text, Timestamptz, Varchar};
use dotenv::dotenv;
use std::env;
use crate::schema::users;
use crate::schema::watches;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub github_user: String
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub github_user: &'a str,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub async fn list_users(connection: &mut PgConnection) -> Vec<User> {

    use crate::schema::users::dsl::*;

    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());

    results
}
pub async fn create_user(connection: &mut PgConnection, github_user: &str) -> User {

    use crate::schema::users;

    let new_user = NewUser { github_user: &github_user };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")
}

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