#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
use serde::{Deserialize, Serialize};
mod db;
use db::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize, Deserialize)]
struct MyObj<'a> {
    name: &'a str,
}

async fn index() -> impl Responder {
    let files = get_files();
    HttpResponse::Ok().json(&files)
}

async fn upload() -> impl Responder {
    insert_files();
    HttpResponse::Ok().json(MyObj {
        name: "Mantab bang",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .route("/", web::get().to(index))
            .route("/uploadfiles", web::post().to(upload))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
