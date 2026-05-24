use actix_web::web;
use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthenticationDto
{
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,
    #[serde(rename = "TenantId")]
    pub tenant_id: String,
    #[serde(rename = "BaseUrl")]
    pub base_url: String,
    #[serde(rename = "RedirectUrl")]
    pub redirect_url: String,
}

pub struct ConfigProvider
{
    pub config: Config,
}

impl ConfigProvider
{
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("src/appsettings.json")).build()?;

        Ok(ConfigProvider { config: settings })
    }
    pub fn get_connection_string(&self) -> Result<String, ConfigError>
    {
        let secret = self.config.get_string("ConnectionStrings.Database")?;

        Ok(secret)
    }

    pub fn get_authentication_settings(&self) -> Result<AuthenticationDto, ConfigError>
    {
        Ok(self.config.get::<AuthenticationDto>("Authentication")?)
    }
}
