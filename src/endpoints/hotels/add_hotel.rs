use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::domain::hotel::Hotel;
use crate::persistence::repositories::hotel_repository::HotelRepository;


#[derive(Deserialize, utoipa::ToSchema)]
pub struct AddHotelRequest
{
    pub name: String,
    pub description: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub rating: i32,
    pub check_in_time: String,
    pub check_out_time: String
}

#[utoipa::path(
    post,
    path = "/api/hotels",
    request_body = AddHotelRequest,
    responses(
        (
            status = CREATED,
            description = "Hotel successfully created"
        ),
        (
            status = OK,
            description = "Idempotent successfully"
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid hotel payload supplied"
        )
    ),
    tag = "Hotels"
)]
#[post("api/hotels")]
pub async fn add_hotel(repo: web::Data<HotelRepository>, payload: web::Json<AddHotelRequest>) -> impl Responder
{
    let hotel = match Hotel::new(
        &payload.name,
        &payload.description,
        &payload.address,
        &payload.city,
        &payload.country,
        payload.rating,
        &payload.check_in_time,
        &payload.check_out_time)
    {
        Ok(hotel) => hotel,
        Err(e) => return HttpResponse::BadRequest().body(e.0)
    };
    
    match repo.hotel_exists(&hotel.name).await
    {
        Ok(true) => return HttpResponse::Ok().finish(),
        Ok(false) => (),
        Err(e) => return  HttpResponse::BadRequest().body(e.to_string())
    };

    match repo.add_hotel(hotel).await {
        Ok(_) => HttpResponse::Created().into(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}