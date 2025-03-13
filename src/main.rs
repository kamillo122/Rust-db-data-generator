mod db;
mod models;
mod utils;

use db::database_handler::{clear_staff, generate_staff, get_staff};
use db::mongodb::connect_mongodb;
use db::mysql::connect_mysql;

use axum::{extract::Extension, routing::post, Router};
use http::header::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Failed to load .env file: {:?}", e);
        return;
    }
    let mysql_url = dotenvy::var("MYSQL_URL").expect("MYSQL_URL must be set");
    let mongodb_uri = dotenvy::var("MONGODB_URI").expect("MONGODB_URI must be set");

    println!("🔍 Connecting to MySQL...");
    let pool = match connect_mysql(&mysql_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to MySQL: {:?}", e);
            return;
        }
    };

    println!("🔍 Connecting to MongoDB...");
    let mongodb_client = match connect_mongodb(&mongodb_uri).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Failed to connect to MongoDB: {:?}", e);
            return;
        }
    };
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/generate", post(generate_staff))
        .route("/clear", post(clear_staff))
        .route("/staff", post(get_staff))
        .layer(Extension(pool.clone()))
        .layer(Extension(mongodb_client.clone()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    pool.disconnect().await.unwrap();
    mongodb_client.shutdown().await;
}
