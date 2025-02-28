mod db;
mod models;

use db::mongodb::{connect_mongodb, insert_many_into_mongodb};
use db::mysql_handler::{connect_mysql, insert_staff_batch};
use models::staff::Staff;
use std::time::Instant;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Failed to load .env file: {:?}", e);
        return;
    }
    let mysql_url = dotenvy::var("MYSQL_URL").expect("MYSQL_URL must be set");
    let mongodb_uri = dotenvy::var("MONGODB_URI").expect("MONGODB_URL must be set");

    println!("Connecting to MySQL...");
    let pool = match connect_mysql(&mysql_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to MySQL: {:?}", e);
            return;
        }
    };
    println!("Successfully connected to Mysql!");

    println!("Connecting to MongoDB...");
    let mongodb_client = match connect_mongodb(&mongodb_uri).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {:?}", e);
            return;
        }
    };
    let start = Instant::now();
    let staff_list: Vec<Staff> = Staff::generate_batch(5);

    // Wkładanie do MySQL w jednym batchu
    if let Err(e) = insert_staff_batch(&pool, staff_list.clone()).await {
        eprintln!("MySQL batch insert error: {:?}", e);
    }

    // Wkładanie do MongoDB w jednym batchu
    if let Err(e) = insert_many_into_mongodb(&mongodb_client, &staff_list).await {
        eprintln!("MongoDB batch insert error: {:?}", e);
    }
    let duration = start.elapsed();
    println!("Time elapsed in generator is: {:?}", duration);
    pool.disconnect().await.unwrap();
}
