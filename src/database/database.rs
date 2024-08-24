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

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

