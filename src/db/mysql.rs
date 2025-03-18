use crate::db::table_type::{GetParams, TableType};
use axum::Json;
use chrono::NaiveDate;
use mysql_async::prelude::*;
use mysql_async::{Conn, Error, Opts, Pool};

use crate::models::{
    address::Address, client::Client, contract::Contract, employee::Employee, payment::Payment,
    project::Project, task::Task, technology::Technology,
};

pub async fn connect_mysql(database_url: &str) -> Result<Pool, Error> {
    let opts = Opts::from_url(database_url)?;
    let pool = Pool::new(opts);

    let mut conn: Conn = pool.get_conn().await?;
    conn.query_drop("SELECT 1").await?;

    println!("✅ Successfully connected to MySQL!");
    Ok(pool)
}

pub async fn fetch_all_data_mysql(
    pool: &Pool,
    table_name: String,
) -> Result<Json<Vec<TableType>>, Error> {
    let mut conn = pool.get_conn().await?;
    match table_name.as_str() {
        "address" => {
            let address_list: Vec<TableType> = conn
                .query_map(
                    "SELECT city, street, street_number, postal_code FROM address",
                    |(city, street, street_number, postal_code): (
                        String,
                        String,
                        String,
                        String,
                    )| {
                        TableType::Address(Address {
                            city,
                            street,
                            street_number,
                            postal_code,
                        })
                    },
                )
                .await?;
            Ok(Json(address_list))
        }
        "client" => {
            let client_list: Vec<TableType> = conn
                .query_map(
                    "SELECT first_name, last_name, email, phone_number FROM client",
                    |(first_name, last_name, email, phone_number): (
                        String,
                        String,
                        String,
                        String,
                    )| {
                        TableType::Client(Client {
                            first_name,
                            last_name,
                            email,
                            phone_number,
                        })
                    },
                )
                .await?;
            Ok(Json(client_list))
        }
        "contract" => {
            let contract_list: Vec<TableType> = conn
                .query_map(
                    "SELECT type_of_contract, start_date, end_date, salary FROM contract",
                    |(type_of_contract, start_date, end_date, salary): (
                        String,
                        String,
                        String,
                        i32,
                    )| {
                        TableType::Contract(Contract {
                            type_of_contract,
                            start_date: NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap(),
                            end_date: NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap(),
                            salary,
                        })
                    },
                )
                .await?;
            Ok(Json(contract_list))
        }
        "employee" => {
            let employee_list: Vec<TableType> = conn
                .query_map(
                    "SELECT first_name, last_name, phone_number, email, position, contract_date FROM employee",
                |(first_name, last_name, phone_number, email, position, contract_date): (
                String,
                String,
                String,
                String,
                String,
                String,
            )| {
                TableType::Employee(Employee {
                    first_name,
                    last_name,
                    email,
                    phone_number,
                    position,
                    contract_date: NaiveDate::parse_from_str(&contract_date, "%Y-%m-%d").unwrap(),
                })
            },
        )
    .await?;
            Ok(Json(employee_list))
        }
        "payment" => {
            let payment_list: Vec<TableType> = conn
                .query_map(
                    "SELECT amount, payment_due_date, method FROM payment",
                    |(amount, payment_due_date, method): (f32, String, String)| {
                        TableType::Payment(Payment {
                            amount,
                            payment_due_date: NaiveDate::parse_from_str(
                                &payment_due_date,
                                "%Y-%m-%d",
                            )
                            .unwrap(),
                            method,
                        })
                    },
                )
                .await?;
            Ok(Json(payment_list))
        }
        "project" => {
            let project_list: Vec<TableType> = conn
                .query_map(
                    "SELECT name, description, start_date, end_date, status FROM project",
                    |(name, description, start_date, end_date, status): (
                        String,
                        String,
                        String,
                        String,
                        String,
                    )| {
                        TableType::Project(Project {
                            name,
                            description,
                            start_date: NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap(),
                            end_date: NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap(),
                            status,
                        })
                    },
                )
                .await?;
            Ok(Json(project_list))
        }
        "task" => {
            let task_list: Vec<TableType> = conn
                .query_map(
                    "SELECT name, description, start_date, end_date, status FROM task",
                    |(name, description, start_date, end_date, status): (
                        String,
                        String,
                        String,
                        String,
                        String,
                    )| {
                        TableType::Task(Task {
                            name,
                            description,
                            start_date: NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap(),
                            end_date: NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap(),
                            status,
                        })
                    },
                )
                .await?;
            Ok(Json(task_list))
        }
        "technology" => {
            let technology_list: Vec<TableType> = conn
                .query_map(
                    "SELECT name, description FROM technology",
                    |(name, description): (String, String)| {
                        TableType::Technology(Technology { name, description })
                    },
                )
                .await?;
            Ok(Json(technology_list))
        }
        _ => Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid table name",
        ))),
    }
}

pub async fn clear_mysql(
    pool: &Pool,
    table_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
    match table_name.as_str() {
        "address" => {
            conn.query_drop("DELETE FROM address").await?;
        }
        "client" => {
            conn.query_drop("DELETE FROM client").await?;
        }
        "contract" => {
            conn.query_drop("DELETE FROM contract").await?;
        }
        "employee" => {
            conn.query_drop("DELETE FROM employee").await?;
        }
        "payment" => {
            conn.query_drop("DELETE FROM payment").await?;
        }
        "project" => {
            conn.query_drop("DELETE FROM project").await?;
        }
        "task" => {
            conn.query_drop("DELETE FROM task").await?;
        }
        "technology" => {
            conn.query_drop("DELETE FROM technology").await?;
        }
        _ => return Err("Invalid table name".into()),
    }
    Ok(())
}

pub async fn insert_batch(pool: &Pool, list: &Vec<TableType>) -> Result<(), Error> {
    let mut conn = pool.get_conn().await?;
    let mut tx = conn
        .start_transaction(mysql_async::TxOpts::default())
        .await?;

    let mut queries: Vec<(&str, Vec<mysql_async::Params>)> = Vec::new();

    for item in list {
        match item {
            TableType::Technology(_) => queries.push((
                "INSERT IGNORE INTO technology (name, description) VALUES (?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Task(_) => queries.push((
                "INSERT IGNORE INTO task (name, description, start_date, end_date, status) VALUES (?, ?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Project(_) => queries.push((
                "INSERT IGNORE INTO project (name, description, start_date, end_date, status) VALUES (?, ?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Payment(_) => queries.push((
                "INSERT IGNORE INTO payment (amount, payment_due_date, method) VALUES (?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Employee(_) => queries.push((
                "INSERT IGNORE INTO employee (first_name, last_name, email, phone_number, position, contract_date) VALUES (?, ?, ?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Contract(_) => queries.push((
                "INSERT IGNORE INTO contract (type_of_contract, start_date, end_date, salary) VALUES (?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Client(_) => queries.push((
                "INSERT IGNORE INTO client (first_name, last_name, email, phone_number) VALUES (?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
            TableType::Address(_) => queries.push((
                "INSERT IGNORE INTO address (city, street, street_number, postal_code) VALUES (?, ?, ?, ?)",
                vec![
                    item.get_params()
                ]
            )),
        }
    }

    for (query, params) in queries {
        tx.exec_batch(query, params).await?;
    }

    tx.commit().await?;
    Ok(())
}
