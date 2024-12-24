mod home;
mod utils;

use actix_web::{App, HttpServer};

use utils::settings::get_address;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = get_address();
    HttpServer::new(|| App::new().configure(home::urls::routes))
        .bind(address)?
        .run()
        .await
}
