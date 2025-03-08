use crate::models::staff::Staff;
use axum::{extract::Extension, response::IntoResponse, Json};
use chrono::NaiveDate;
use mysql_async::prelude::*;
use mysql_async::{Conn, Error, Opts, Pool};
use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    count: usize,
}

pub async fn connect_mysql(database_url: &str) -> Result<Pool, Error> {
    let opts = Opts::from_url(database_url)?;
    let pool = Pool::new(opts);

    let mut conn: Conn = pool.get_conn().await?;
    conn.query_drop("SELECT 1").await?;

    println!("✅ Successfully connected to MySQL!");
    Ok(pool)
}

async fn fetch_all_staff(pool: &Pool) -> Result<Json<Vec<Staff>>, Error> {
    let mut conn = pool.get_conn().await?;

    let staff_list: Vec<Staff> = conn
        .query_map(
            "SELECT name, department, salary, phone, hire_date FROM staff",
            |(name, department, salary, phone, hire_date): (
                String,
                String,
                i32,
                String,
                String,
            )| Staff {
                name,
                department,
                salary,
                phone,
                hire_date: NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d").unwrap(),
            },
        )
        .await?;

    Ok(Json(staff_list))
}

pub async fn generate_staff(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<String>, String> {
    let names = Staff::load_names_from_file("src/utils/names.txt");
    let staff_list: Vec<Staff> = Staff::generate_batch(payload.count, &names);

    insert_staff_batch(&pool, &staff_list)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(format!(
        "✅ Generated and inserted {} staff",
        payload.count
    )))
}

pub async fn clear_staff(Extension(pool): Extension<Pool>) -> Result<Json<String>, String> {
    let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
    conn.query_drop("DELETE FROM staff")
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json("✅ Database cleared successfully".to_string()))
}

pub async fn get_staff(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    match fetch_all_staff(&pool).await {
        Ok(staff) => staff.into_response(),
        Err(e) => {
            eprintln!("❌ Failed to fetch staff: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch staff",
            )
                .into_response()
        }
    }
}

pub async fn insert_staff_batch(
    pool: &Pool,
    staff_list: &Vec<Staff>,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut conn = pool.get_conn().await?;

    let mut tx = conn
        .start_transaction(mysql_async::TxOpts::default())
        .await?;
    let values: Vec<mysql_async::Params> = staff_list
        .into_iter()
        .map(|staff| {
            mysql_async::Params::Positional(vec![
                staff.name.clone().into(),
                staff.department.clone().into(),
                staff.salary.clone().into(),
                staff.phone.clone().into(),
                staff
                    .hire_date
                    .clone()
                    .format("%Y-%m-%d")
                    .to_string()
                    .into(),
            ])
        })
        .collect();

    tx.exec_batch(
        r"INSERT IGNORE INTO staff (name, department, salary, phone, hire_date) 
        VALUES (?, ?, ?, ?, ?)",
        values,
    )
    .await?;

    tx.commit().await?;

    let duration = start.elapsed();
    println!("🚀 Time elapsed by MySQL batch insert: {:?}", duration);

    Ok(())
}
