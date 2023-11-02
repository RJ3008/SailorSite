#[allow(unused_imports)]
use actix_web::{get, web::*, App, http::*, HttpServer, Responder, HttpResponse};
#[allow(unused_imports)]
use std::fs;
mod paths;
use paths::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(index)
    .service(about)
    .service(js)
    .service(cssr)
    .service(test)
    .service(images))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
