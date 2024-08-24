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