use crate::db::table_type::TableType;
use axum::{extract::Extension, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use super::mongodb::{clear_mongodb, fetch_all_data_mongodb, insert_batch_mongodb};
use super::mysql::{clear_mysql, fetch_all_data_mysql, insert_batch};

use mysql_async::Pool;

use crate::models::{
    address::Address, client::Client, contract::Contract, employee::Employee, payment::Payment,
    project::Project, task::Task, technology::Technology,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateRequest {
    count: usize,
    db_type: String,
    table_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClearRequest {
    db_type: String,
    table_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequest {
    db_type: String,
    table_name: String,
}

pub async fn generate_data(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<mongodb::Client>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = match payload.db_type.as_str() {
        "mysql" => {
            match payload.table_name.as_str() {
                "employee" => {
                    let employees = Employee::generate_batch(payload.count);
                    let employees: Vec<TableType> =
                        employees.into_iter().map(TableType::Employee).collect();
                    insert_batch(&pool, &employees)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "client" => {
                    let clients = Client::generate_batch(payload.count);
                    let clients: Vec<TableType> =
                        clients.into_iter().map(TableType::Client).collect();
                    insert_batch(&pool, &clients)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "address" => {
                    let addresses = Address::generate_batch(payload.count);
                    let addresses: Vec<TableType> =
                        addresses.into_iter().map(TableType::Address).collect();
                    insert_batch(&pool, &addresses)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "contract" => {
                    let contracts = Contract::generate_batch(payload.count);
                    let contracts: Vec<TableType> =
                        contracts.into_iter().map(TableType::Contract).collect();
                    insert_batch(&pool, &contracts)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "payment" => {
                    let payments = Payment::generate_batch(payload.count);
                    let payments: Vec<TableType> =
                        payments.into_iter().map(TableType::Payment).collect();
                    insert_batch(&pool, &payments)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "project" => {
                    let projects = Project::generate_batch(payload.count);
                    let projects: Vec<TableType> =
                        projects.into_iter().map(TableType::Project).collect();
                    insert_batch(&pool, &projects)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "task" => {
                    let tasks = Task::generate_batch(payload.count);
                    let tasks: Vec<TableType> = tasks.into_iter().map(TableType::Task).collect();
                    insert_batch(&pool, &tasks)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "technology" => {
                    let technologies = Technology::generate_batch(payload.count);
                    let technologies: Vec<TableType> = technologies
                        .into_iter()
                        .map(TableType::Technology)
                        .collect();
                    insert_batch(&pool, &technologies)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                _ => return Err((StatusCode::BAD_REQUEST, "❌ Invalid table name".to_string())),
            }
            Ok(Json(format!("✅ Generated {}", payload.count)))
        }
        "mongodb" => {
            match payload.table_name.as_str() {
                "employee" => {
                    let employees = Employee::generate_batch(payload.count);
                    let employees: Vec<TableType> =
                        employees.into_iter().map(TableType::Employee).collect();
                    insert_batch_mongodb(&mongodb_client, &employees)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "client" => {
                    let clients = Client::generate_batch(payload.count);
                    let clients: Vec<TableType> =
                        clients.into_iter().map(TableType::Client).collect();
                    insert_batch_mongodb(&mongodb_client, &clients)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "address" => {
                    let addresses = Address::generate_batch(payload.count);
                    let addresses: Vec<TableType> =
                        addresses.into_iter().map(TableType::Address).collect();
                    insert_batch_mongodb(&mongodb_client, &addresses)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "contract" => {
                    let contracts = Contract::generate_batch(payload.count);
                    let contracts: Vec<TableType> =
                        contracts.into_iter().map(TableType::Contract).collect();
                    insert_batch_mongodb(&mongodb_client, &contracts)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "payment" => {
                    let payments = Payment::generate_batch(payload.count);
                    let payments: Vec<TableType> =
                        payments.into_iter().map(TableType::Payment).collect();
                    insert_batch_mongodb(&mongodb_client, &payments)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "project" => {
                    let projects = Project::generate_batch(payload.count);
                    let projects: Vec<TableType> =
                        projects.into_iter().map(TableType::Project).collect();
                    insert_batch_mongodb(&mongodb_client, &projects)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "task" => {
                    let tasks = Task::generate_batch(payload.count);
                    let tasks: Vec<TableType> = tasks.into_iter().map(TableType::Task).collect();
                    insert_batch_mongodb(&mongodb_client, &tasks)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                "technology" => {
                    let technologies = Technology::generate_batch(payload.count);
                    let technologies: Vec<TableType> = technologies
                        .into_iter()
                        .map(TableType::Technology)
                        .collect();
                    insert_batch_mongodb(&mongodb_client, &technologies)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                _ => return Err((StatusCode::BAD_REQUEST, "❌ Invalid table name".to_string())),
            }
            Ok(Json(format!("✅ Generated {}", payload.count)))
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "❌ Invalid database type".to_string(),
            ))
        }
    };
    result.map(|_| Json(format!("✅ Generated {}", payload.count)))
}

pub async fn clear_staff(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<mongodb::Client>,
    Json(payload): Json<ClearRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    match payload.db_type.as_str() {
        "mysql" => clear_mysql(&pool, payload.table_name)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        "mongodb" => clear_mongodb(&mongodb_client, &payload.table_name)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "❌ Invalid database type".to_string(),
            ))
        }
    }
    Ok(Json(format!("✅ Cleared {}", payload.db_type)))
}

pub async fn get_data(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<mongodb::Client>,
    Json(payload): Json<GetRequest>,
) -> Result<Json<Vec<TableType>>, (StatusCode, String)> {
    match payload.db_type.as_str() {
        "mysql" => fetch_all_data_mysql(&pool, payload.table_name)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        "mongodb" => fetch_all_data_mongodb(&mongodb_client, &payload.table_name)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "❌ Invalid database type".to_string(),
        )),
    }
}
