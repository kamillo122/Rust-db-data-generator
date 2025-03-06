use crate::models::staff::Staff;
use mongodb::{
    bson::doc,
    bson::to_bson,
    bson::Document,
    error::Result,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use std::time::Instant;

pub async fn connect_mongodb(uri: &str) -> Result<Client> {
    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("test")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("✅ Successfully connected to MongoDB!");

    Ok(client)
}

pub async fn insert_into_mongodb(client: &Client, staff: &Staff) -> Result<()> {
    let database = client.database("test");
    let collection = database.collection("staff");

    let doc = doc! {
        "name": &staff.name,
        "department": &staff.department,
        "salary": staff.salary,
        "phone": &staff.phone,
        "hire_date": staff.hire_date.to_string(),
    };

    collection.insert_one(doc, None).await?;

    Ok(())
}

pub async fn insert_many_into_mongodb(client: &Client, staff_list: &[Staff]) -> Result<()> {
    let start = Instant::now();
    let database = client.database("test");
    let collection = database.collection("staff");

    let docs: Vec<Document> = staff_list
        .iter()
        .map(|staff| to_bson(staff).unwrap().as_document().unwrap().clone())
        .collect();

    collection.insert_many(docs, None).await?;

    let duration = start.elapsed();
    println!("🚀 Time elapsed by MongoDB batch insert: {:?}", duration);
    Ok(())
}
