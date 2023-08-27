use zero2prod::prisma::subscriptions;

use crate::helpers::{sapwn_prisma, spawn_app};

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
