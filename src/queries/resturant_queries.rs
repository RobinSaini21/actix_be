#[path ="../db/db_connection.rs"] mod db_connection;
use mongodb::{Client, Database, bson::Document};

pub async fn get_demo() {
  let client = db_connection::db_demo().await;
  if let Ok(client) = client {
    // Access the "sample_restaurants" database
    let db: Database = client.database("sample_restaurants");

    // Now you can work with the "sample_restaurants" database
    // For example, you can list the collections in the database
    let collections = db.list_collection_names(None).await;

    match collections {
        Ok(names) => {
            for name in names {
                println!("Collection: {}", name);
            }
        }
        Err(err) => {
            eprintln!("Error listing collections: {}", err);
        }
    }
} else {
    eprintln!("Error connecting to the database");
}
}