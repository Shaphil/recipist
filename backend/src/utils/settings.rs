use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::env::VarError;

lazy_static! {
    pub static ref HOST: String = get_host();
    pub static ref PORT: u16 = get_port();
}

// application defaults
const _HOST: &str = "127.0.0.1";
const _PORT: u16 = 8080;

// read keys from the `.env` file
fn get_env(key: &str) -> Result<String, VarError> {
    dotenv().ok();
    env::var(key)
}

fn get_host() -> String {
    get_env("HOST").unwrap_or(_HOST.to_string())
}

fn get_port() -> u16 {
    let port = get_env("PORT").unwrap_or(_PORT.to_string());
    port.parse::<u16>().unwrap_or(_PORT)
}

pub fn get_address() -> (String, u16) {
    let host = get_host();
    let port = get_port();

    (host, port)
}
