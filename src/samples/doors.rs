// use std::sync::Arc;
// use axum::extract::State;
// use axum::http::StatusCode;
// use axum::Json;
// use axum::response::IntoResponse;
//
// use serde::{Serialize, Deserialize};
// use sqlx::{PgPool, Row};
//
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Door {
//     pub isbn: String,
//     pub name: String,
//     pub description: String,
// }
//
// pub async fn get_doors () -> impl IntoResponse {
//     let query = "SELECT isbn, name, description FROM doors";
//     let result = sqlx::query(query);
//
//     let one = Door {
//         isbn: "local".to_string(),
//         name: "local".to_string(),
//         description: "local".to_string()
//     };
//     // let response = vec![one];
//     // Json(response)
//     (StatusCode::NOT_FOUND, format!("Actually we found {}", one.isbn))
// }
// pub async fn create_door (door: &Door, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
//     let query = "INSERT INTO doors (isbn, name, description) VALUES ($1, $2, $3);";
//     sqlx::query(query)
//         .bind(&door.isbn)
//         .bind(&door.name)
//         .bind(&door.description)
//         .execute(pool)
//         .await
//         .expect("::Error::Failed to create DOOR::");
//     Ok(())
// }
// pub async fn update_door (door: &Door, isbn: &str, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
//     let query = "UPDATE door SET name = $1, price = $2, description = $3 WHERE isbn = $4";
//     sqlx::query(query)
//         .bind(&door.name)
//         .bind(&door.description)
//         .bind(&door.isbn)
//         .execute(pool)
//         .await
//         .expect("::Error:: Failed to update DOOR::");
//     Ok(())
// }
// pub async fn read_isbn (pool: &PgPool) -> Result<Door, Box<dyn std::error::Error>> {
//     let query = "SELECT isbn, name, description FROM door";
//     let request = sqlx::query(query);
//
//     // fetch_one(pool) -> pulling a single row
//     let row = request.fetch_one(pool).await.expect("::Error:: Failed to read DOOR::");
//
//     // fetch_optional(pool) -> similar to fetch_one but firstly checks if a row exists and then returns an option
//     // let maybe_row = request.fetch_optional(pool).await.expect("...");
//     // let result = maybe_row.map(|row| {
//     //     Door {
//     //         isbn: row.get("isbn"),
//     //         name: row.get("name"),
//     //         description: row.get("description"),
//     //         price: 00.00
//     //     }
//     // });
//     // or
//     // let instance = match maybe_row {
//     //     Ok(value) => Door {
//     //         isbn: value.get("isbn"),
//     //         name: value.get("name"),
//     //         description: value.get("description"),
//     //         price: 00.00
//     //     },
//     //     Err(e) => Door {
//     //         isbn: String::from("no"),
//     //         name: String::from("no"),
//     //         description: String::from("no"),
//     //         price: 00.00
//     //     }
//     // };
//
//     // fetch_all(pool) -> pulling all rows
//     // let rows = request.fetch_all(pool).await.expect("...");
//     // let results = rows.iter().map(|row| {
//     //     Door {
//     //         isbn: row.get("isbn"),
//     //         name: row.get("name"),
//     //         description: row.get("description"),
//     //         price: 00.00
//     //     }
//     // }).collect();
//
//     // fetch(pool) -> more async way (streaming), working with large data helps to optimise fetching
//     // let mut rows = request.fetch(pool);
//     // let mut doors: Vec<Door> = vec![];
//     // while let Some(row) = rows.try_next().await.expect("...") {    <-- use futures::TryStreamExt; is required
//     //     doors.push(Door {
//     //         isbn: row.get("isbn"),
//     //         name: row.get("name"),
//     //         description: row.get("description"),
//     //         price: 00.00
//     //     })
//     // }
//
//     let result = Door {
//         isbn: row.get("isbn"),
//         name: row.get("name"),
//         description: row.get("description"),
//     };
//     Ok(result)
// }