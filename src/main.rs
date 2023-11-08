use email_newsletter::{configuration::get_configuration, startup::run};
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Can't load config.");
    let connection_pool = PgPool::connect(&configuration.database.connection_url())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // let _ = listener.local_addr().unwrap().port();
    run(listener, connection_pool)?.await
}
