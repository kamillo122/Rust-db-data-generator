use axum::{extract::Extension, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use super::mongodb::{clear_mongodb, fetch_all_staff_mongodb, insert_many_into_mongodb};
use super::mysql::{clear_staff_mysql, fetch_all_staff_mysql, insert_staff_batch};
use crate::models;
use crate::utils;

use models::staff::Staff;
use mongodb::Client;
use mysql_async::Pool;
use utils::utils::load_from_file;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateRequest {
    count: usize,
    db_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClearRequest {
    db_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetStaffRequest {
    db_type: String,
}

pub async fn generate_staff(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<Client>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let names = load_from_file("src/utils/names.txt");
    let staff_list: Vec<Staff> = Staff::generate_batch(payload.count, &names);

    match payload.db_type.as_str() {
        "mysql" => {
            insert_staff_batch(&pool, &staff_list)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        "mongodb" => {
            insert_many_into_mongodb(&mongodb_client, &staff_list)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        "both" => {
            // let (mysql_result, mongo_result) = join!(
            //     insert_staff_batch(&pool, &staff_list),
            //     insert_many_into_mongodb(&mongodb_client, &staff_list)
            // );

            // if let Err(e) = mysql_result {
            //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            // }
            // if let Err(e) = mongo_result {
            //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            // }
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "❌ Invalid database type".to_string(),
            ))
        }
    }
    Ok(Json("✅ Staff generated successfully".to_string()))
}

pub async fn clear_staff(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<Client>,
    Json(payload): Json<ClearRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    match payload.db_type.as_str() {
        "mysql" => {
            clear_staff_mysql(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        "mongodb" => {
            clear_mongodb(&mongodb_client)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        "both" => {
            // let (mysql_result, mongo_result) =
            //     join!(clear_staff_mysql(&pool), clear_mongodb(&mongodb_client));

            // if let Err(e) = mysql_result {
            //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            // }
            // if let Err(e) = mongo_result {
            //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            // }
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "❌ Invalid database type".to_string(),
            ))
        }
    }

    Ok(Json(format!("✅ Cleared {}", payload.db_type)))
}

pub async fn get_staff(
    Extension(pool): Extension<Pool>,
    Extension(mongodb_client): Extension<Client>,
    Json(payload): Json<GetStaffRequest>,
) -> Result<Json<Vec<Staff>>, (StatusCode, String)> {
    match payload.db_type.as_str() {
        "mysql" => fetch_all_staff_mysql(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        "mongodb" => fetch_all_staff_mongodb(&mongodb_client)
            .await
            .map(Json)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "❌ Invalid database type".to_string(),
        )),
    }
}
