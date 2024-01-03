use crate::{domain::ValidEmail, email_client::EmailClient};
use config::{Config, ConfigError, File, FileFormat};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use tracing;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

impl ApplicationSettings {
    pub fn host_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub username: String,
    pub password: Secret<String>,
    pub require_ssl: bool, // Set connection to be encrypted or not
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer // try an encrypted connection
        };
        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
            .log_statements(tracing::log::LevelFilter::Trace)
    }
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
}

impl EmailClientSettings {
    pub fn email_client(&self) -> EmailClient {
        let email_address =
            ValidEmail::parse(self.sender_email.clone()).expect("Invalid sener email address");
        EmailClient::new(self.base_url.clone(), email_address)
    }
}

fn pathbuf_to_string(path: std::path::PathBuf) -> String {
    path.into_os_string()
        .into_string()
        .expect("Error converting to String")
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialize our configuration reader
    let working_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = working_path.join("config");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let base_file = pathbuf_to_string(configuration_directory.join("base"));
    let env_file = pathbuf_to_string(configuration_directory.join(environment.as_str()));

    let settings = Config::builder()
        .add_source(File::new(base_file.as_str(), FileFormat::Yaml).required(true))
        .add_source(File::new(env_file.as_str(), FileFormat::Yaml).required(true))
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `prodution`.",
                other
            )),
        }
    }
}
