extern crate core;

use actix_web::{App, HttpServer};
use actix_web::cookie::Key;
use actix_web::web::{service, Data};
use utoipa_actix_web::AppExt;

use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use endpoints::authentication::callback::CallbackRequest;
use endpoints::hotels::add_hotel::AddHotelRequest;
use endpoints::hotels::get_hotels::GetHotelsRequest;
use endpoints::hotels::get_hotels::HotelDto;
use endpoints::room_types::add_room_type::AddRoomTypeRequest;

pub mod endpoints;
pub mod providers;
pub mod persistence;
pub mod domain;
pub mod middlewares;

#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            AddHotelRequest,
            AddRoomTypeRequest,
            CallbackRequest,
            GetHotelsRequest,
            HotelDto
        )
    ),
    tags(
        (name = "Hotels", description = "Hotel management endpoints."),
        (name = "Room Types", description = "Room type management endpoints."),
        (name = "Authentication", description = "Authentication and authorization endpoints.")
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

    let config_provider = providers::config_provider::ConfigProvider::new().expect("Failed to init config provider");
    let connection_string = config_provider.get_connection_string().expect("Failed to get connection string");
    let auth_settings = config_provider.get_authentication_settings().expect("Failed to get authentication settings");
    
    let pg_pool = providers::pg_provider::get_database_pool(&connection_string).await.expect("Failed to get database pool");
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::session_middleware::build_cookie_session_middleware(secret_key.clone()))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(Data::new(persistence::repositories::hotel_repository::HotelRepository::new(pg_pool.clone())))
            .app_data(Data::new(persistence::repositories::room_type_repository::RoomTypeRepository::new(pg_pool.clone())))
            .app_data(Data::new(providers::auth_provider::AuthProvider::new(&auth_settings.client_id, &auth_settings.client_secret, &auth_settings.base_url, &auth_settings.tenant_id, &auth_settings.redirect_url).expect("Failed to create auth provider")))
            .service(endpoints::authentication::login::login)
            .service(endpoints::authentication::callback::callback)
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


