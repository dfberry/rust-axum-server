use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
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