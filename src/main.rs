use axum::{Extension, Router};
use axum::routing::{get, post};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
// use crate::samples::doors::get_doors;

mod samples;
mod config;
mod customerror;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = config::Config::new();
    // let db_url = "postgres://postgres:123@localhost:5432/store";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
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
        // .route("/doors", get(samples::door::get_doors).post(samples::door::create_door))
        // .route("/windows", post(samples::windows::create_window).get(samples::windows::get_windows))
        .route("/custom", get(samples::types::get_call).post(samples::types::post_call).delete(samples::types::delete_call))
        .layer(Extension(pool.clone()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
