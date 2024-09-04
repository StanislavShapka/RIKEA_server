// use std::str::FromStr;
// use axum::{Extension, Json};
// use axum::response::IntoResponse;
// use serde::{Deserialize, Serialize};
// use sqlx::{Error, FromRow, Row};
// use sqlx::postgres::PgRow;
// use sqlx::postgres::types::PgMoney;
// use crate::customerror::CustomError;
// #[derive(Deserialize, Serialize)]
// pub struct Window {
//     pub name: String,
//     pub description: String,
//     pub price: PgMoney
// }
// impl FromRow<'_, PgRow> for Window {
//     fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
//         let name = row.try_get("name")?;
//         let description = row.try_get("description")?;
//         let money: PgMoney = row.try_get("price")?;
//         let locale_frac_digits = 2;
//         let price = money.to_bigdecimal(locale_frac_digits);
//         Ok(Window {name, description, price})
//     }
// }
//
// pub async fn create_window(Extension(pool): Extension<sqlx::PgPool>, Json(payload): Json<Window>) -> impl IntoResponse {
//     let query = "insert into windows (name, description, price) values ($1, $2, $3)";
//     sqlx::query(&query)
//         .bind(&payload.name)
//         .bind(&payload.description)
//         .bind(&payload.price)
//         .execute(&pool)
//         .await
//         .expect("Error from POST WINDOWS");
//     Json(202)
// }
//
// pub async fn get_windows(Extension(pool): Extension<sqlx::PgPool>) -> Result<Json<Vec<Window>>, CustomError> {
//     let query = "select * from windows";
//     let vec: Vec<Window> = sqlx::query_as(&query).fetch_all(&pool).await.expect("ERROR");
//     Ok(Json(vec))
// }
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
