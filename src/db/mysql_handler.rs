use crate::models::staff::Staff;
use mysql_async::{Error, Opts, Pool, TxOpts};
use mysql_async::prelude::*;

pub async fn connect_mysql(database_url: &str) -> Result<Pool, Error> {
    let opts = Opts::from_url(database_url)?;
    let pool = Pool::new(opts);
    Ok(pool)
}

async fn staff_exists(pool: &Pool, id: i32) -> bool {
    let mut conn = pool.get_conn().await.unwrap();
    let result: Option<i32> = conn
        .exec_first("SELECT 1 FROM staff WHERE id = ?", (id,))
        .await
        .unwrap_or(None);
    result.is_some()
}

pub async fn insert_into_mysql(pool: &Pool, staff: &Staff) -> Result<(), Error> {
    let mut conn = pool.get_conn().await?;

    if staff_exists(&pool, staff.id).await {
        println!("Skipping duplicate MySQL entry: {:?}", staff);
        return Ok(());
    }
    let mut tx = conn.start_transaction(TxOpts::default()).await?;
    tx.exec_drop(
        "INSERT INTO staff (id, name, department, salary, phone, hire_date) VALUES (?, ?, ?, ?, ?, ?)",
        (
            &staff.id, 
            &staff.name, 
            &staff.department, 
            &staff.salary, 
            &staff.phone, 
            &staff.hire_date.to_string(),
        )
    )
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn insert_staff_batch(pool: &Pool, staff_list: Vec<Staff>) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get_conn().await?;
    
    let mut tx = conn.start_transaction(mysql_async::TxOpts::default()).await?;

    let values: Vec<mysql_async::Params> = staff_list.into_iter().map(|staff| {
        mysql_async::Params::Positional(vec![
            staff.id.into(),
            staff.name.into(),
            staff.department.into(),
            staff.salary.into(),
            staff.phone.into(),
            staff.hire_date.format("%Y-%m-%d").to_string().into(),
        ])
    }).collect();

    tx.exec_batch(
        r"INSERT INTO staff (id, name, department, salary, phone, hire_date) 
          VALUES (?, ?, ?, ?, ?, ?)",
        values,
    ).await?;

    tx.commit().await?;
    
    Ok(())
}