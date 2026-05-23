use config::{Config, ConfigError};

pub fn get_connection_string() -> Result<String, ConfigError>
{
    let settings = Config::builder()
        .add_source(config::File::with_name("src/appsettings.json")).build()?;

    let secret = settings.get_string("ConnectionStrings.Database")?;

    Ok(secret)
}
