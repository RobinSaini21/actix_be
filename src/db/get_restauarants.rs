use std::borrow::Borrow;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions ,  Collection, bson::doc};
use serde::{Serialize, Deserialize};
use tokio;
use mongodb::error::Error as MongoError;

// don't forget this!
use futures::stream::{ StreamExt , TryStreamExt , TryStream };


#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}

// async fn find_handler() -> impl Responder {
//     // Connect to MongoDB
//     let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
//     let client = Client::with_options(client_options).unwrap();

//     // Access the specific database and collection
//     let db = client.database("your_database_name");
//     let collection = db.collection::<Restaurant>("your_collection_name");

//     // Define your query (example: find all documents)
//     let query = mongodb::bson::doc! {};

//     // Execute the find query
//     let cursor = collection.find(query, None).await.unwrap();

//     // Convert the results to a vector of your document type
//     let results: Vec<Result<Restaurant, _>> = cursor.try_collect().await.unwrap();

//     // Extract the documents from the Ok results
//     let documents: Vec<Restaurant> = results
//         .into_iter()
//         .filter_map(Result::ok)
//         .collect();

//     // Convert the results to JSON and return as an HTTP response
//     HttpResponse::Ok().json(documents)
// }


pub async fn db_demo() -> Result<Vec<Restaurant>, MongoError> {
    let mut res: Vec<Restaurant> = Vec::new();
    let uri = "mongodb+srv://robinsaini2126:MLcB98hSZmTIQTWS@cluster0.x83bzir.mongodb.net/?retryWrites=true&w=majority";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");
    let mut cursor = my_coll.find(
        doc! { "cuisine": "French" },
        None
    ).await?;
    while let Some(doc) = cursor.try_next().await? {
        res.push(doc);
    }
    Ok(res)
}
pub async fn get_restauarants() -> HttpResponse {
    let user_detail = db_demo().await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}