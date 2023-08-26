use std::net::TcpListener;

use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, prisma::PrismaClient};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let prisma_client: PrismaClient = PrismaClient::_builder().build().await.unwrap();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    run(listener, prisma_client, email_client)?.await?;
    Ok(())
}
