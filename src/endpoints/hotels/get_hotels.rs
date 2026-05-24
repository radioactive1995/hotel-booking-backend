use actix_web::{get, Responder, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use crate::persistence::repositories::hotel_repository::HotelRepository;

#[serde_as]
#[derive(Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct GetHotelsRequest {
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String>>")]
    pub names: Option<Vec<String>>,
    pub from_rating: Option<i32>,
    pub to_rating: Option<i32>
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HotelDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub rating: Option<i32>,
    pub check_in_time: String,
    pub check_out_time: String
}

#[utoipa::path(
    get,
    path = "/api/hotels",
    params(GetHotelsRequest),
    responses(
        (
            status = OK,
            description = "Hotels successfully retrieved",
            body = Vec<HotelDto>
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid query parameters supplied"
        )
    ),
    tag = "Hotels"
)]
#[get("api/hotels")]
pub async fn get_hotels(repo: web::Data<HotelRepository>, request: web::Query<GetHotelsRequest>) -> impl Responder
{
    let from_rating = request.from_rating.unwrap_or(0);
    let to_rating = request.to_rating.unwrap_or(5);
    let names = request.names.clone().unwrap_or(vec![]);

    match repo.get_hotels_by_names_and_rating(&names, from_rating, to_rating).await {
        Ok(hotels) =>
            {
                let hotel_dtos: Vec<HotelDto> = hotels
                    .iter()
                    .map(|h| HotelDto {
                        id: h.id,
                        name: h.name.clone(),
                        description: h.description.clone(),
                        address: h.address.clone(),
                        city: h.city.clone(),
                        country: h.country.clone(),
                        rating: h.rating,
                        check_in_time: h.check_in_time.to_string(),
                        check_out_time: h.check_out_time.to_string(),
                    }).collect();
                HttpResponse::Ok().json(hotel_dtos)
            },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())}
    }