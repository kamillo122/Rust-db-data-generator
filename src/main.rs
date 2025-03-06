mod db;
mod models;

use db::mongodb::{connect_mongodb, insert_many_into_mongodb};
use db::mysql_handler::{connect_mysql, get_staff, insert_staff_batch};
use models::staff::Staff;

use mongodb::Client;
use mysql_async::Pool;
use std::time::Instant;

use axum::{extract::Extension, routing::get, Router};

use tokio::join;

async fn run(pool: &Pool, mongodb_client: &Client) {
    let start = Instant::now();
    let names = Staff::load_names_from_file("src/utils/names.txt");
    let staff_list: Vec<Staff> = Staff::generate_batch(250, &names);
    let duration = start.elapsed();
    println!("🚀 Time elapsed by generator: {:?}", duration);

    let (mysql_result, mongo_result) = join!(
        insert_staff_batch(pool, &staff_list),
        insert_many_into_mongodb(mongodb_client, &staff_list)
    );

    if let Err(e) = mysql_result {
        eprintln!("❌ MySQL batch insert error: {:?}", e);
    }
    if let Err(e) = mongo_result {
        eprintln!("❌ MongoDB batch insert error: {:?}", e);
    }

    println!(
        "📝 Inserted {:?} records into MySQL and MongoDB",
        staff_list.len()
    );
}

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
    run(&pool, &mongodb_client).await;
    let app = Router::new()
        .route("/staff", get(get_staff))
        .layer(Extension(pool.clone()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    pool.disconnect().await.unwrap();
    mongodb_client.shutdown().await;
}
