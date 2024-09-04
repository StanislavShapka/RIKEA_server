use axum::{Router};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use crate::samples::doors::get_doors;

pub mod samples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let db_url = "postgres://postgres:123@localhost:5432/store";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool");

    let allowed = AllowOrigin::list(vec![
       "http://localhost:5173".parse().unwrap(),
       "http://localhost:5174".parse().unwrap()
    ]);
    let cors = CorsLayer::new()
        .allow_origin(allowed)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/doors", get(get_doors))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
