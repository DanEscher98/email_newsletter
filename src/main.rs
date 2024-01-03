use email_newsletter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Can't load config.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let email_client = configuration.email_client.email_client();

    let address = configuration.application.host_address();
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await
}
