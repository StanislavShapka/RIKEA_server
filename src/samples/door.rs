use axum::{Extension, Json};
use axum::response::IntoResponse;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use crate::customerror::CustomError;
use sqlx::{FromRow, query_as};
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Door {
    isbn: i32,
    name: String,
    description: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateDoor {
    name: String,
    description: String
}
pub async fn get_doors(Extension(pool): Extension<sqlx::PgPool>) -> Result<Json<Vec<Door>>, CustomError> {
    let query = "select * from doors;";
    let res: Vec<Door> = query_as(&query).fetch_all(&pool).await.expect("ERROR");
    Ok(Json(res))
}

pub async fn create_door(Extension(pool): Extension<sqlx::PgPool>, Json(payload): Json<CreateDoor>) -> impl IntoResponse {
    let query = "insert into doors (name, description) values($1, $2)";
    sqlx::query(&query)
        .bind(&payload.name)
        .bind(&payload.description)
        .execute(&pool)
        .await
        .expect("ERROR by POST DOOR");
    Json(202)
}