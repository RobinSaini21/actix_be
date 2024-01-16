use mongodb::{ 
    bson::doc,
    Client,
    Collection
};
use futures::TryStreamExt;
use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}

pub async fn db_demo() -> mongodb::error::Result<()> {
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
        println!("{:?}", doc);
    }
    Ok(())
}