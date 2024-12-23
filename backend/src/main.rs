use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": ".:: Welcome to Actix Web ::."}))
}

#[post("/echo")]
async fn echo(message: String) -> impl Responder {
    HttpResponse::Ok().body(message)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = ("127.0.0.1", 8080);
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(echo)
    })
        .bind(address)?
        .run()
        .await
}
