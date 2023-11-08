use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_url_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialize our configuration reader
    let config = Config::builder()
        .add_source(File::new("configuration", FileFormat::Yaml))
        .build()?;

    config.try_deserialize::<Settings>()
}
