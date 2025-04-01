use crate::db::table_type_mysql::TableType;
use crate::models;
use crate::models::{
    address::Address, contract::Contract, employee::Employee, payment::Payment, project::Project,
    task::Task, technology::Technology,
};
use axum::Json;
use chrono::NaiveDate;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    error::Result,
    options::ClientOptions,
    Client, Collection,
};

pub async fn connect_mongodb(uri: &str) -> Result<Client> {
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    client
        .database("soft")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("✅ Successfully connected to MongoDB!");
    Ok(client)
}

pub async fn fetch_all_data_mongodb(
    client: &Client,
    collection_name: &str,
) -> Result<Json<Vec<TableType>>> {
    let database = client.database("soft");
    let collection: Collection<Document> = database.collection(collection_name);
    let mut cursor = collection.find(None, None).await?;
    let mut results = Vec::new();

    while let Some(doc) = cursor.next().await {
        if let Ok(doc) = doc {
            if let Ok(entry) = mongodb_doc_to_table_type(collection_name, doc.clone()) {
                results.push(entry);
            } else {
                println!("❌ Nie udało się sparsować: {:?}", doc);
            }
        }
    }

    Ok(Json(results))
}

fn mongodb_doc_to_table_type(collection_name: &str, doc: Document) -> Result<TableType> {
    match collection_name {
        "address" => {
            if let Some(Bson::Document(address_doc)) = doc.get("Address") {
                let city = address_doc.get_str("city").unwrap_or_default().to_string();
                let street = address_doc
                    .get_str("street")
                    .unwrap_or_default()
                    .to_string();
                let street_number = address_doc
                    .get_str("street_number")
                    .unwrap_or_default()
                    .to_string();
                let postal_code = address_doc
                    .get_str("postal_code")
                    .unwrap_or_default()
                    .to_string();

                Ok(TableType::Address(Address {
                    city,
                    street,
                    street_number,
                    postal_code,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Address field",
                )))
            }
        }
        "client" => {
            if let Some(Bson::Document(client_doc)) = doc.get("Client") {
                let first_name = client_doc
                    .get_str("first_name")
                    .unwrap_or_default()
                    .to_string();
                let last_name = client_doc
                    .get_str("last_name")
                    .unwrap_or_default()
                    .to_string();
                let email = client_doc.get_str("email").unwrap_or_default().to_string();
                let phone_number = client_doc
                    .get_str("phone_number")
                    .unwrap_or_default()
                    .to_string();

                Ok(TableType::Client(models::client::Client {
                    first_name,
                    last_name,
                    email,
                    phone_number,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Client field",
                )))
            }
        }
        "contract" => {
            if let Some(Bson::Document(contract_doc)) = doc.get("Contract") {
                let type_of_contract = contract_doc
                    .get_str("type_of_contract")
                    .unwrap_or_default()
                    .to_string();
                let start_date_str = contract_doc.get_str("start_date").unwrap_or_default();
                let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let end_date_str = contract_doc.get_str("end_date").unwrap_or_default();
                let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let salary: i32 = contract_doc
                    .get_str("salary")
                    .unwrap_or("0")
                    .parse::<i32>()
                    .unwrap_or(0);

                Ok(TableType::Contract(Contract {
                    type_of_contract,
                    start_date,
                    end_date,
                    salary,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Contract field",
                )))
            }
        }
        "employee" => {
            if let Some(Bson::Document(employee_doc)) = doc.get("Employee") {
                let first_name = employee_doc
                    .get_str("first_name")
                    .unwrap_or_default()
                    .to_string();
                let last_name = employee_doc
                    .get_str("last_name")
                    .unwrap_or_default()
                    .to_string();
                let email = employee_doc
                    .get_str("email")
                    .unwrap_or_default()
                    .to_string();
                let phone_number = employee_doc
                    .get_str("phone_number")
                    .unwrap_or_default()
                    .to_string();
                let position = employee_doc
                    .get_str("position")
                    .unwrap_or_default()
                    .to_string();
                let contract_date_str = employee_doc.get_str("contract_date").unwrap_or_default();
                let contract_date = NaiveDate::parse_from_str(contract_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

                Ok(TableType::Employee(Employee {
                    first_name,
                    last_name,
                    email,
                    phone_number,
                    position,
                    contract_date,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Employee field",
                )))
            }
        }
        "payment" => {
            if let Some(Bson::Document(payment_doc)) = doc.get("Payment") {
                let amount_str = payment_doc.get_str("amount").unwrap_or("0");
                let amount = amount_str.parse::<f32>().unwrap_or(0.0);
                let payment_due_date_str =
                    payment_doc.get_str("payment_due_date").unwrap_or_default();
                let payment_due_date = NaiveDate::parse_from_str(payment_due_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let method = payment_doc
                    .get_str("method")
                    .unwrap_or_default()
                    .to_string();

                Ok(TableType::Payment(Payment {
                    amount,
                    payment_due_date,
                    method,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Payment field",
                )))
            }
        }
        "project" => {
            if let Some(Bson::Document(project_doc)) = doc.get("Project") {
                let name = project_doc.get_str("name").unwrap_or_default().to_string();
                let description = project_doc
                    .get_str("description")
                    .unwrap_or_default()
                    .to_string();
                let start_date_str = project_doc.get_str("start_date").unwrap_or_default();
                let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let end_date_str = project_doc.get_str("end_date").unwrap_or_default();
                let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let status = project_doc
                    .get_str("status")
                    .unwrap_or_default()
                    .to_string();

                Ok(TableType::Project(Project {
                    name,
                    description,
                    start_date,
                    end_date,
                    status,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Project field",
                )))
            }
        }
        "task" => {
            if let Some(Bson::Document(task_doc)) = doc.get("Task") {
                let name = task_doc.get_str("name").unwrap_or_default().to_string();
                let description = task_doc
                    .get_str("description")
                    .unwrap_or_default()
                    .to_string();
                let start_date_str = task_doc.get_str("start_date").unwrap_or_default();
                let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let end_date_str = task_doc.get_str("end_date").unwrap_or_default();
                let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")
                    .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
                let status = task_doc.get_str("status").unwrap_or_default().to_string();

                Ok(TableType::Task(Task {
                    name,
                    description,
                    start_date,
                    end_date,
                    status,
                }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Task field",
                )))
            }
        }
        "technology" => {
            if let Some(Bson::Document(technology_doc)) = doc.get("Technology") {
                let name = technology_doc
                    .get_str("name")
                    .unwrap_or_default()
                    .to_string();
                let description = technology_doc
                    .get_str("description")
                    .unwrap_or_default()
                    .to_string();

                Ok(TableType::Technology(Technology { name, description }))
            } else {
                Err(mongodb::error::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing Technology field",
                )))
            }
        }
        _ => Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unknown collection",
        ))),
    }
}

pub async fn clear_mongodb(client: &Client, collection_name: &str) -> Result<()> {
    let database = client.database("soft");
    let collection: Collection<Document> = database.collection(collection_name);
    collection.delete_many(doc! {}, None).await?;
    Ok(())
}

pub async fn insert_batch_mongodb(client: &Client, list: &Vec<TableType>) -> Result<()> {
    let database = client.database("soft");
    let mut collections: Vec<(&str, Vec<Document>)> = Vec::new();

    for item in list {
        let collection_name = match item {
            TableType::Address(_) => "address",
            TableType::Client(_) => "client",
            TableType::Contract(_) => "contract",
            TableType::Employee(_) => "employee",
            TableType::Payment(_) => "payment",
            TableType::Project(_) => "project",
            TableType::Task(_) => "task",
            TableType::Technology(_) => "technology",
        };

        let doc = to_bson(item)?.as_document().cloned().unwrap();
        if let Some((_, docs)) = collections
            .iter_mut()
            .find(|(name, _)| *name == collection_name)
        {
            docs.push(doc);
        } else {
            collections.push((collection_name, vec![doc]));
        }
    }

    for (collection_name, docs) in collections {
        let collection = database.collection::<Document>(collection_name);
        collection.insert_many(docs, None).await?;
    }

    Ok(())
}
