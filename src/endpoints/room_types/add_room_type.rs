use actix_web::{post, web, HttpResponse, Responder};
use rust_decimal::Decimal;
use serde::{Deserialize};
use rust_decimal::{serde::arbitrary_precision};
use crate::domain::room_type::RoomType;
use crate::persistence::repositories::hotel_repository::HotelRepository;
use crate::persistence::repositories::room_type_repository::RoomTypeRepository;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AddRoomTypeRequest
{
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    #[serde(with = "arbitrary_precision")]
    #[schema(value_type = f64, example = 129.99)]
    pub base_price: Decimal,
    pub total_rooms: i32
}

#[utoipa::path(
    post,
    path = "/api/hotels/{id}/room-type",
    request_body = AddRoomTypeRequest,
    params(
        (
            "id" = i32,
            Path,
            description = "Hotel identifier to add the room type to",
            example = 10
        )
    ),
    responses(
        (
            status = CREATED,
            description = "Room type successfully added to hotel"
        ),
        (
            status = OK,
            description = "Room type already exists"
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid room type payload supplied"
        ),
        (
            status = NOT_FOUND,
            description = "No hotel exists with the provided id"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error"
        )
    ),
    tag = "Room Types"
)]
#[post("api/hotels/{id}/room-type")]
pub async fn add_room_type(
    hotel_repo: web::Data<HotelRepository>,
    room_type_repo: web::Data<RoomTypeRepository>,
    payload: web::Json<AddRoomTypeRequest>,
    path: web::Path<i32>) -> impl Responder
{
    let room_type = match RoomType::new(
        &payload.name,
        payload.description.as_deref(),
        payload.capacity,
        payload.base_price,
        payload.total_rooms) {
        Ok(room_type) => room_type,
        Err(e) => return HttpResponse::BadRequest().body(e.0)
    };

    let hotel_id = path.into_inner();

    match hotel_repo.hotel_exists_by_id(hotel_id).await {
        Ok(true) => {}
        Ok(false) => return HttpResponse::NotFound().body(format!("No hotel exists with provided id: {}", hotel_id)),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    }

    match room_type_repo.room_type_exists_by_name(&payload.name).await
    {
        Ok(true) => return HttpResponse::Ok().into(),
        Ok(false) => (),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    }

    match room_type_repo.add_room_type_to_hotel(room_type, hotel_id).await {
        Ok(_) => HttpResponse::Created().into(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}