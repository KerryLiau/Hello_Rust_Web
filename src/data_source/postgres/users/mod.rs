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
    let mut tx = conn.begin().await?;
    let query = "SELECT * FROM users WHERE id = $1";
    let user = sqlx::query_as::<_, Entity>(query)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            let error = match e {
                Error::RowNotFound => ApiError::NotFound(format!("No user found for id '{id}'")),
                _ => ApiError::InternalServerError(e.to_string()),
            };
            tracing::warn!("{}", error.message());
            error
        })?;
    tx.commit().await.map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    Ok(user)
}

// #[tracing::instrument(skip(conn))]
// pub async fn update_by_id(id: i32, conn: &Pool<Postgres>) -> Result<Entity, ApiError> {
//     let mut tx = conn.begin().await?;
//     let query = "UPDATE users SET ... WHERE id = $1";
//     let user = sqlx::query_as::<_, Entity>(query)
//         .bind(id)
//         .fetch_one(&mut *tx)
//         .await?;
//     tx.commit().await.map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     Ok(user)
// }