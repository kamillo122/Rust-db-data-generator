use crate::models::staff::Staff;
use axum::{extract::Extension, response::IntoResponse, Json};
use chrono::NaiveDate;
use mysql_async::prelude::*;
use mysql_async::{Conn, Error, Opts, Pool, TxOpts};
use std::time::Instant;

pub async fn connect_mysql(database_url: &str) -> Result<Pool, Error> {
    let opts = Opts::from_url(database_url)?;
    let pool = Pool::new(opts);

    let mut conn: Conn = pool.get_conn().await?;
    conn.query_drop("SELECT 1").await?;

    println!("✅ Successfully connected to MySQL!");
    Ok(pool)
}

// pub async fn insert_into_mysql(pool: &Pool, staff: &Staff) -> Result<(), Error> {
//     let mut conn = pool.get_conn().await?;

//     let mut tx = conn.start_transaction(TxOpts::default()).await?;
//     tx.exec_drop(
//         "INSERT INTO staff (name, department, salary, phone, hire_date) VALUES (?, ?, ?, ?, ?)",
//         (
//             &staff.name,
//             &staff.department,
//             &staff.salary,
//             &staff.phone,
//             &staff.hire_date.to_string(),
//         ),
//     )
//     .await?;

//     tx.commit().await?;

//     Ok(())
// }

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
