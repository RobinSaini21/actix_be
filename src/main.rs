mod controllers {
    pub(crate) mod json_body_handler_demo;
}

mod db {
    pub(crate) mod mongodb_connection;
    pub(crate) mod get_restauarants;
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use controllers::json_body_handler_demo::index_user;
use db::{mongodb_connection::db_demo, get_restauarants::get_restauarants};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // if let Err(err) = db_demo().await {
    //     eprintln!("Error: {}", err);
    // }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/add-user", web::post().to(index_user))
            .route("/get-restauarants", web::get().to(get_restauarants))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}