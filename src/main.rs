use email_newsletter::configuration::get_configuration;
// use email_newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Can't load config.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // let _ = listener.local_addr().unwrap().port();
    email_newsletter::run(listener)?.await
}
