use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let _ = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener);

    server?.await
}
