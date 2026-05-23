use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
pub mod endpoints;
pub mod providers;
pub mod persistence;
pub mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pg_pool = providers::pg_provider::get_database_pool().await.expect("Failed to get database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(persistence::repositories::hotel_repository::HotelRepository::new(pg_pool.clone())))
            .service(endpoints::hotels::get_hotels::endpoint)
            .service(endpoints::hotels::add_hotel::endpoint)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


