use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

use darim::routes;

/// Health check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let address = env::var("ADDRESS").expect("ADDRESS not found");
    let cert_file_path = env::var("TLS_CERT_FILE_PATH").expect("TLS_CERT_FILE_PATH not found");
    let key_file_path = env::var("TLS_KEY_FILE_PATH").expect("TLS_KEY_FILE_PATH not found");

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(cert_file_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_file_path).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    println!("Server running at {}", address);

    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
            .configure(routes::auth::init_routes)
    })
    .bind_rustls(address, config)?
    .run()
    .await
}
