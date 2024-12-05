use crate::database::page::{PagedResult, PageRequest};

use crate::schema::osb_session;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

pub async fn create_new_user_session(
    connection: &mut PgConnection,
    user_id: &str,
    access_token: &str,
) -> Result<(), diesel::result::Error> {

    use crate::schema::osb_session::dsl::*;

    let new_user_session = NewUserSession {
        user_id,
        access_token,
    };

    diesel::insert_into(osb_session)
        .values(&new_user_session)
        .execute(connection)?;

    Ok(())
}
pub async fn delete_user_session(
    connection: &mut PgConnection,
    user_id: &str,
) -> Result<(), diesel::result::Error> {
    use crate::schema::osb_session::dsl::*;

    diesel::delete(osb_session.filter(user_id.eq(user_id)))
        .execute(connection)?;

    Ok(())
}
pub async fn get_user_session(
    connection: &mut PgConnection,
    user_id: &str,
) -> Option<UserSession> {
    use crate::schema::osb_session::dsl::*;

    let sessions = osb_session
        .filter(user_id.eq(user_id))
        .limit(1)
        .select(UserSession::as_select())
        .load(connection)
        .expect("Error loading sessions");

    // consume the Vec<User> and return the first item directly
    if let Some(session) = sessions.into_iter().next() {
        println!("Display session: {:?}", session);
        Some(session)
    } else {
        println!("No session found with user_id: {}", user_id);
        None
    }
}