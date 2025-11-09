use crate::data_source::postgres::users::Entity;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub id: i32,
    pub age: i32,
    pub name: String,
    pub gender: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl IntoResponse for Resp {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Entity> for Resp {
    fn from(value: Entity) -> Self {
        Resp {
            id: value.id,
            age: value.age,
            name: format!("{} {}", value.f_name, value.l_name),
            gender: value.gender,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_user_resp() {
        let user = Entity {
            id: 1,
            age: 20,
            f_name: "John".to_string(),
            l_name: "Doe".to_string(),
            gender: "male".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let resp = Resp::from(user);
        println!("{:?}", resp);
        assert_eq!(resp.name, "John Doe");
    }
}