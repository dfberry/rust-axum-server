use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;


pub fn establish_connection() -> PgConnection {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if database_url.is_empty() {
        panic!("DATABASE_URL is empty. Please set a valid database URL.");
    }
    match PgConnection::establish(&database_url) {
        Ok(conn) => {
            println!("Successfully connected to {}", database_url);
            return conn
        }
        Err(e) => {
            println!("Error connecting to {}: {}", database_url, e);
            std::process::exit(1);
        }
    }
}

