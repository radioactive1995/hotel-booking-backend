use sqlx::{Error, PgPool};
use crate::domain::room_type::RoomType;

pub struct RoomTypeRepository {
    pub pg_pool: PgPool,
}

impl RoomTypeRepository {

    pub fn new(pg_pool: PgPool) -> Self { RoomTypeRepository { pg_pool } }
    pub async fn add_room_type_to_hotel(&self, room_type: RoomType, hotel_id: i32) -> Result<(), Error>
    {
        sqlx::query!(r#"
            insert into room_types
                (hotel_id, name, description, capacity, base_price, total_rooms)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
        hotel_id,
        room_type.name,
        room_type.description,
        room_type.capacity,
        room_type.base_price,
        room_type.total_rooms).execute(&self.pg_pool).await?;

        Ok(())
    }

    pub async fn room_type_exists_by_name(&self, name: &str) -> Result<bool, sqlx::Error>
    {
        let record = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM room_types WHERE name = $1) AS "exists!"
            "#,
            name)
            .fetch_one(&self.pg_pool).await?;

        Ok(record.exists)
    }
}