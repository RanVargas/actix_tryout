use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::user::User;

use crate::db::DbError;

pub fn find_user_by_uid(
    conn: &mut PgConnection,
    uid: Uuid,
) -> Result<Option<User>, DbError> {
    use crate::models::user_schema::users::dsl::{users, id, *};

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    conn: &mut PgConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<User, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::models::user_schema::users::dsl::{users, id, *};

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        email: format!("{}@example.com", nm),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}