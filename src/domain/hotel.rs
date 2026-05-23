use time::macros::{format_description, time};
use sqlx::types::time::Time;
use crate::domain::common::errors::DomainError;

pub struct Hotel
{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub rating: Option<i32>,
    pub check_in_time: Time,
    pub check_out_time: Time
}

impl Hotel {
    pub fn new(
        name: &str,
        description: &str,
        address: &str,
        city: &str,
        country: &str,
        rating: i32,
        check_in_time: &str,
        check_out_time: &str) -> Result<Self, DomainError>
    {
        if rating > 5 || rating <= 0
        {
            return Err(DomainError("rating must be between 5 and 0".to_string()));
        }

        let format = format_description!("[hour]:[minute]:[second]");

        let parsed_check_in_time = Time::parse(check_in_time, &format).map_err(|e| DomainError(e.to_string()))?;
        let parsed_check_out_time = Time::parse(check_out_time, &format).map_err(|e| DomainError(e.to_string()))?;
        
        //let parsed_check_in_time = Time::parse_from_str(&check_in_time, "%H:%M:%S").map_err(|e| DomainError(e.to_string()))?;
        //let parsed_check_out_time = NaiveTime::parse_from_str(&check_out_time, "%H:%M:%S").map_err(|e| DomainError(e.to_string()))?;
        
        Ok(Hotel {
            id: 0,
            name: name.into(),
            description: Some(description.into()),
            address: address.into(),
            city: city.into(),
            country : country.into(),
            rating: Some(rating),
            check_in_time: parsed_check_in_time,
            check_out_time: parsed_check_out_time
        })
    }
}