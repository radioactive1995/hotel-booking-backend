use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::domain::hotel::Hotel;
use crate::persistence::repositories::hotel_repository::HotelRepository;


#[derive(Deserialize)]
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

#[post("api/hotels")]
pub async fn endpoint(repo: web::Data<HotelRepository>, payload: web::Json<AddHotelRequest>) -> impl Responder {

    let domain_result = Hotel::new(
        &payload.name,
        &payload.description,
        &payload.address,
        &payload.city,
        &payload.country,
        payload.rating,
        &payload.check_in_time,
        &payload.check_out_time,
    ).map_err(|e| { HttpResponse::BadRequest().body(e.0) });

    if domain_result.is_err() { return domain_result.err().unwrap(); }
    let hotel = domain_result.unwrap();
    
    let exists_result = repo.hotel_exists(&hotel.name).await
        .map_err(|e| { HttpResponse::BadRequest().body(e.to_string()) });
    
    match exists_result {
        Ok(true) => return HttpResponse::Ok().finish(),
        Ok(false) => (),
        Err(e) => return e
    };
    
    let result = repo.add_hotel(hotel)
        .await.map_err( |e| HttpResponse::InternalServerError().body(e.to_string()));
    
    if result.is_err() { return result.err().unwrap(); }

    HttpResponse::Created().into()
}