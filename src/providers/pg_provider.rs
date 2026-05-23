use sqlx::{PgPool, postgres::PgPoolOptions, Error};
use sqlx::postgres::PgConnectOptions;
use crate::providers::config_provider;

pub async fn get_database_pool() -> Result<PgPool, Error> {
    
    let connection_string = config_provider::get_connection_string().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await?;
    
    Ok(pool)
}