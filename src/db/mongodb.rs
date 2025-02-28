use crate::models::staff::Staff;
use mongodb::{
    bson::doc,
    bson::Document,
    error::Result,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};

pub async fn connect_mongodb(uri: &str) -> Result<Client> {
    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("test")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Successfully connected to MongoDB!");

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
    let database = client.database("test");
    let collection: Collection<Document> = database.collection("staff");

    let mut new_staff: Vec<Staff> = Vec::new();
    for staff in staff_list {
        if !staff_exists(client, staff.id).await? {
            new_staff.push(staff.clone());
        } else {
            println!("Skipping duplicate MongoDB entry: {:?}", staff);
        }
    }

    let docs: Vec<_> = new_staff
        .iter()
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
    collection.insert_many(docs, None).await?;

    Ok(())
}
