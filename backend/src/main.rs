mod home;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = ("127.0.0.1", 8080);
    HttpServer::new(|| App::new().configure(home::urls::routes))
        .bind(address)?
        .run()
        .await
}
