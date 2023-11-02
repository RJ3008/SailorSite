#[allow(unused_imports)]
use actix_web::{get, web::*, App, http::*, HttpServer, Responder, HttpResponse};
use std::fs;

#[get("/")]
    pub async fn index() -> HttpResponse {
    let index = fs::read_to_string("sailor/index.html").unwrap();
    let headering = fs::read_to_string("sailor/headering/header.html").unwrap(); //Header

    let mainscript = format!("<!DOCTYPE html><html><head><title>SaiolrOS</title></head><body>{}{}</body>", headering, index );
    
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(mainscript)
        .into()
}

#[get("/javas/{filename}")]
pub async fn js(filename: Path<String>) -> HttpResponse {
    let javascript_file = fs::read_to_string(format!("sailor/js/{}", filename)).unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/javascript")
        .body(javascript_file)
        .into()
}
#[get("/css/{filename}")]
pub async fn cssr(filename: Path<String>) -> HttpResponse {
    let cssser = fs::read_to_string(format!("sailor/css/{}", filename)).unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/javascript")
        .body(cssser)
        .into()
}

#[get("/images/header/logo.png")]
pub async fn images() -> HttpResponse {
    let logo = fs::read("sailor/img/logo.png").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(logo)
        .into()
}

#[get("/products")]
pub async fn about() -> HttpResponse {
    let products = fs::read_to_string("sailor/products.hmtl").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(products)
        .into()
}
#[get("/test")]
pub async fn test() -> HttpResponse {
    let testhead = fs::read_to_string("sailor/headering/header.html").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(testhead)
        .into()
}