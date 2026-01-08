use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = crate::models::user_schema::users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
