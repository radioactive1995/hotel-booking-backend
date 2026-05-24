use crate::domain::common::errors::DomainError;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct RoomType {
    pub id: i32,
    pub hotel_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub base_price: Decimal,
    pub total_rooms: i32
}

impl RoomType {
    pub fn new(
        name: &str,
        description: Option<&str>,
        capacity: i32,
        base_price: Decimal,
        total_rooms: i32) ->  Result<RoomType, DomainError>
    {
        if name.trim().len() == 0
        {
            return Err(DomainError("Name must not be empty".to_string()));
        }

        if total_rooms <= 0 || total_rooms > 3
        {
            return Err(DomainError("The total rooms must be not larger than 3 and atleast 1".to_string()))
        }

        if base_price <= dec!(0.0)
        {
            return Err(DomainError("Base price must be greater than 0".to_string()))
        }

        if capacity <= 0
        {
            return Err(DomainError("Capacity must be greater than 0".to_string()))
        }

        Ok(RoomType 
        {
            id: 0,
            hotel_id: 0,
            name: name.into(),
            description: description.map(|e| e.into()), 
            capacity,
            base_price: base_price.into(),
            total_rooms
        })
    }
}