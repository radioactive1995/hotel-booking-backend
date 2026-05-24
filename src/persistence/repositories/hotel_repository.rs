use sqlx::{PgPool};
use crate::domain::hotel::Hotel;
use crate::persistence;

pub struct HotelRepository {
    pub pg_pool: PgPool,
}

impl HotelRepository {
    pub fn new(pg_pool: PgPool) -> Self { HotelRepository { pg_pool } }

    pub async fn add_hotel(&self, hotel: Hotel) -> Result<(), sqlx::Error>
    {
        sqlx::query!(
            r#"
            INSERT INTO hotels (
                name,
                description,
                address,
                city,
                country,
                rating,
                check_in_time,
                check_out_time)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            hotel.name,
            hotel.description,
            hotel.address,
            hotel.city,
            hotel.country,
            hotel.rating,
            hotel.check_in_time,
            hotel.check_out_time)
            .execute(&self.pg_pool).await?;

        Ok(())
    }

    pub async fn hotel_exists_by_name(&self, name: &str) -> Result<bool, sqlx::Error>
    {
        let record = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM hotels WHERE name = $1) AS "exists!"
            "#,
            name)
            .fetch_one(&self.pg_pool).await?;

        Ok(record.exists)
    }

    pub async fn hotel_exists_by_id(&self, id: i32) -> Result<bool, sqlx::Error>
    {
        let record = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM hotels WHERE id = $1) AS "exists!"
            "#,
            id)
            .fetch_one(&self.pg_pool).await?;

        Ok(record.exists)
    }

    pub async fn get_hotels_by_names_and_rating(&self, names: &[String], from_rating: i32, to_rating: i32) -> Result<Vec<Hotel>, sqlx::Error>
    {
        let result = sqlx::query_as!(
            Hotel,
            r#"
            SELECT *
            FROM hotels
            WHERE
            (
                cardinality($1::text[]) = 0
                OR name = ANY($1::text[])
            )
            AND rating >= $2
            AND rating <= $3
            "#,
            names,
            from_rating,
            to_rating)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(result)
    }
}