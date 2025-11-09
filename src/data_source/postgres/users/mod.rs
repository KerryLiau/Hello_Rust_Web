use crate::core::error::ApiError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Entity {
    pub id: i32,
    pub age: i32,
    pub f_name: String,
    pub l_name: String,
    pub gender: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[tracing::instrument(skip(conn))]
pub async fn get_by_id(id: i32, conn: &Pool<Postgres>) -> Result<Entity, ApiError> {
    let query = "SELECT * FROM users WHERE id = $1";
    let user = sqlx::query_as::<_, Entity>(query)
        .bind(id)
        .fetch_one(conn)
        .await
        .map_err(|e|
            match e {
                Error::RowNotFound => ApiError::NotFound(format!("No user found for id '{id}'")),
                _ => ApiError::InternalServerError(e.to_string()),
            }
        )?;
    Ok(user)
}