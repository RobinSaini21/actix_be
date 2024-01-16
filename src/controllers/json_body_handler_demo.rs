use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    username: String,
}

/// extract `Info` using serde
pub async fn index_user(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

