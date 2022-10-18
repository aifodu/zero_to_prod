//! tests/health_check.rs

use zero_to_prod_2::{run, utils::app_address};

fn spawn_app() -> String {
    let app_address = app_address();

    let server = run(app_address.listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://{}:{}", app_address.ip, app_address.port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
