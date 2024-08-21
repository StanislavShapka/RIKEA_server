use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use crate::samples::doors::{create_door, Door, read_isbn, update_door};

pub mod samples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let db_url = "postgres://postgres:123@localhost:5432/doors";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool");

    // sqlx::migrate!("./migrations").run(&pool).await.expect("::Error::Migration failed");

    // let door1 = Door {
    //     isbn: String::from("1342-en-1329:01"),
    //     name: String::from("Pidoor"),
    //     description: String::from("Best door in UK"),
    //     price: 10.99
    // };
    // let door2 = Door {
    //     isbn: String::from("1342-en-1329:02"),
    //     name: String::from("Damboldoor2"),
    //     description: String::from("Better than nothing2"),
    //     price: 39.50
    // };

    // create_door(&door1, &pool).await.expect("::ERROR:: Failed to invoke fn.create_door");
    // create_door(&door2, &pool).await.expect("::ERROR:: Failed to invoke fn.create_door");

    // update_door(&door2, &door2.isbn, &pool).await.expect("::Error:: Failed to invoke update_door::");
    let response = read_isbn(&pool).await.expect("::Error:: Failed to invoke fn read_isbn::");
    println!("{:?}", &response);

    let allowed = AllowOrigin::list(vec![
       "http://localhost:5173".parse().unwrap(),
       "http://localhost:5174".parse().unwrap()
    ]);
    let cors = CorsLayer::new()
        .allow_origin(allowed)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}