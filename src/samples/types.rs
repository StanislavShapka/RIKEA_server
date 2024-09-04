use axum::{Extension, Json};
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::types::PgMoney;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CustomQuery {
    two: i32
}
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct CustomSend {
    pub one: String,
    pub two: i32,
    pub three: f32,
    pub four: f64,
    pub five: bool,
    pub six: i8,
}
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct CustomReceive {
    pub one: String,
    pub two: i32,
    pub three: f32,
    pub four: f64,
    pub five: bool,
    pub six: String,
    pub seven: chrono::NaiveDateTime
}
pub async fn get_call(Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let query = "select * from custom";
    let mut response: Vec<CustomReceive> = sqlx::query_as(&query).fetch_all(&pool).await.expect("ERROR:: GET CUSTOM");
    Json(response)
}
pub async fn post_call(Extension(pool): Extension<sqlx::PgPool>, Json(payload): Json<CustomSend>) -> impl IntoResponse {
    let query = "insert into custom (one, two, three, four, five, six) values ($1, $2, $3, $4, $5, $6)";
    sqlx::query(&query)
        .bind(&payload.one)
        .bind(&payload.two)
        .bind(&payload.three)
        .bind(&payload.four)
        .bind(&payload.five)
        .bind(&payload.six)
        .execute(&pool)
        .await
        .expect("ERROR:: POST CUSTOM");
    Json(202)
}
pub async fn delete_call(Extension(pool): Extension<sqlx::PgPool>, Query(params): Query<CustomQuery>) -> impl IntoResponse {
    let query = "delete from custom where two = $1";
    sqlx::query(&query).bind(&params.two).execute(&pool).await.expect("ERROR:: DELETE CUSTOM");
    Json(200)
}




























