use crate::database::page::{PagedResult, PageRequest};

use crate::schema::osb_token;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};



pub async fn create_new_user_token(
    connection: &mut PgConnection,
    user_id: &str,
    access_token: &str,
) -> Result<(), diesel::result::Error> {

    use crate::schema::osb_token::dsl::*;

    let new_user_token = NewUserToken {
        user_id,
        access_token,
    };

    diesel::insert_into(osb_token)
        .values(&new_user_token)
        .execute(connection)?;

    Ok(())
}

pub async fn delete_user_token(
    connection: &mut PgConnection,
    user_id: &str,
) -> Result<(), diesel::result::Error> {
    use crate::schema::osb_token::dsl::*;

    diesel::delete(osb_token.filter(user_id.eq(user_id)))
        .execute(connection)?;

    Ok(())
}

pub async fn get_user_token(
    connection: &mut PgConnection,
    user_id: &str,
) -> Option<UserToken> {
    use crate::schema::osb_token::dsl::*;

    let user_tokens = osb_token
        .filter(user_id.eq(user_id))
        .limit(1)
        .select(UserToken::as_select())
        .load(connection)
        .expect("Error loading user tokens");

    // consume the Vec<UserToken> and return the first item directly
    if let Some(user_token) = user_tokens.into_iter().next() {
        Some(user_token)
    } else {
        None
    }
}