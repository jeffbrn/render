use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::fs;

static IMG_DIR: &str = "../output/";

#[get("/")]
async fn index() -> impl Responder {
    let file_path = "static/index.html"; // Replace with the actual image file path
    match fs::read(file_path) {
        Ok(page) => HttpResponse::Ok()
            .content_type("text/html")
            .body(page),
        Err(_) => HttpResponse::NotFound().body("Html page not found"),
    }
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(serde_json::json!({ "message": "Hello, Rust API!" }))
}

#[get("/image")]
async fn get_image() -> impl Responder {
    let file_path = "static/AdmLogo.png"; // Replace with the actual image file path
    match fs::read(file_path) {
        Ok(contents) => HttpResponse::Ok()
            .content_type("image/png")
            .body(contents),
        Err(_) => HttpResponse::NotFound().body("Image not found"),
    }
}

#[get("/output")]
async fn get_output() -> impl Responder {
    let file_path = IMG_DIR.to_owned() + "img.png"; // Replace with the actual image file path
    // println!("Reading image from: {}", file_path);
    match fs::read(file_path) {
        Ok(contents) => HttpResponse::Ok()
            .content_type("image/png")
            .body(contents),
        Err(_) => HttpResponse::NotFound().body("Image not found"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(get_image)
            .service(get_output)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
