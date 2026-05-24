extern crate core;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use utoipa_actix_web::AppExt;

use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use endpoints::hotels::add_hotel::AddHotelRequest;
use endpoints::hotels::get_hotels::GetHotelsRequest;
use endpoints::hotels::get_hotels::HotelDto;

pub mod endpoints;
pub mod providers;
pub mod persistence;
pub mod domain;



#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            AddHotelRequest,
            GetHotelsRequest,
            HotelDto
        )
    ),
    tags(
        (name = "Hotels", description = "Hotel management endpoints.")
    ),
    info(
        title = "Hotel Booking API",
        version = "1.0.0",
        description = "REST API for managing hotels and hotel bookings."
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pg_pool = providers::pg_provider::get_database_pool().await.expect("Failed to get database pool");

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(Data::new(persistence::repositories::hotel_repository::HotelRepository::new(pg_pool.clone())))
            .app_data(Data::new(persistence::repositories::room_type_repository::RoomTypeRepository::new(pg_pool.clone())))
            .service(endpoints::hotels::get_hotels::get_hotels)
            .service(endpoints::hotels::add_hotel::add_hotel)
            .service(endpoints::room_types::add_room_type::add_room_type)
            .openapi_service(|api| SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", api))
            .into_app()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


