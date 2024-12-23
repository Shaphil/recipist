use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": ".:: Welcome to Actix Web ::."}))
}

#[post("/echo")]
pub async fn echo(message: String) -> impl Responder {
    HttpResponse::Ok().body(message)
}
