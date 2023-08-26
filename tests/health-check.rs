//! tests/health-check.rs

use std::net::TcpListener;

use zero2prod::{
    configuration::get_configuration,
    email_client::EmailClient,
    prisma::{
        subscriptions::{self},
        PrismaClient,
    },
    startup::run,
};

pub struct TestApp {
    pub address: String,
    pub prisma_client: PrismaClient,
}

async fn sapwn_prisma() -> PrismaClient {
    PrismaClient::_builder().build().await.unwrap()
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let prisma_client = sapwn_prisma().await;

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

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

    let server = run(listener, prisma_client, email_client).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        prisma_client: sapwn_prisma().await,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arragne
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=test&email=test%40test.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let prisma_client = sapwn_prisma().await;

    let saved = prisma_client
        .subscriptions()
        .find_unique(subscriptions::email::equals("test@test.com".to_owned()))
        .exec()
        .await
        .expect("Failed to execute request.")
        .unwrap();

    assert_eq!(saved.email, "test@test.com");
    assert_eq!(saved.name, "test");

    let _ = prisma_client
        .subscriptions()
        .delete(subscriptions::id::equals(saved.id))
        .exec()
        .await
        .expect("Failed to execute request.");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// #[tokio::test]
// async fn clean_up() {
//     let app = spawn_app().await;
//     let prisma_client = sapwn_prisma().await;
//
//     let _ = prisma_client
//         .subscriptions()
//         .delete_many(vec![])
//         .exec()
//         .await
//         .expect("Failed to execute request.");
// }
