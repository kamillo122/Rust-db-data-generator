use crate::models::staff::Staff;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    bson::Document,
    error::Result,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use std::time::Instant;

const BATCH_SIZE: usize = 100;

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

async fn staff_exists(client: &Client, id: i32) -> Result<bool> {
    let database = client.database("test");
    let collection: Collection<Document> = database.collection("staff");

    let filter = doc! { "id": id };
    let result = collection.find_one(filter, None).await?;

    Ok(result.is_some())
}

pub async fn insert_into_mongodb(client: &Client, staff: &Staff) -> Result<()> {
    if staff_exists(client, staff.id).await? {
        println!("Skipping duplicate MongoDB entry: {:?}", staff);
        return Ok(()); // Skip inserting if already exists
    }

    let database = client.database("test");
    let collection = database.collection("staff");

    let doc = doc! {
        "id": staff.id,
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
    let collection: Collection<Document> = database.collection("staff");

    let cursor = collection
        .find(
            doc! { "id": { "$in": staff_list.iter().map(|s| s.id).collect::<Vec<_>>() } },
            None,
        )
        .await?;

    let existing_ids: Vec<i32> = cursor
        .try_collect::<Vec<Document>>()
        .await?
        .into_iter()
        .filter_map(|doc| doc.get_i32("id").ok())
        .collect();

    let new_staff: Vec<_> = staff_list
        .iter()
        .filter(|staff| !existing_ids.contains(&staff.id))
        .map(|staff| {
            doc! {
                "id": staff.id,
                "name": &staff.name,
                "department": &staff.department,
                "salary": staff.salary,
                "phone": &staff.phone,
                "hire_date": staff.hire_date.to_string(),
            }
        })
        .collect();

    for chunk in new_staff.chunks(BATCH_SIZE) {
        collection.insert_many(chunk.to_vec(), None).await?;
    }
    let duration = start.elapsed();
    println!("🚀 Time elapsed by MongoDB batch insert: {:?}", duration);
    Ok(())
}
