use zero2prod::{configuration::get_configuration, prisma::PrismaClient, startup::Application};

pub struct TestApp {
    pub address: String,
    pub prisma_client: PrismaClient,
}

pub async fn sapwn_prisma() -> PrismaClient {
    PrismaClient::_builder().build().await.unwrap()
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application.");
    let address = format!("http://127.0.0.1:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        prisma_client: sapwn_prisma().await,
    }
}
