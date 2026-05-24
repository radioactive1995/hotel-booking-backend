use sqlx::{PgPool, postgres::PgPoolOptions, Error};

pub async fn get_database_pool(connection_string : &str) -> Result<PgPool, Error>
{
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await?;
    
    Ok(pool)
}