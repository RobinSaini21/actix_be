use mongodb::{Client, bson::Document};

// Assuming Restaurant is a struct representing your data model
#[derive(Debug)]
struct Restaurant {
    // Define your Restaurant struct fields here
    // For example: id: ObjectId, name: String, etc.
}

// Your async function to connect to MongoDB
pub async fn db_demo() -> Result<Client, mongodb::error::Error> {
    let uri = "mongodb+srv://robinsaini2126:MLcB98hSZmTIQTWS@cluster0.x83bzir.mongodb.net/?retryWrites=true&w=majority";

    // Establish a connection to the MongoDB server
    let client = Client::with_uri_str(uri).await?;

    // Return the connected client
    Ok(client)
}
