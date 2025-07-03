use axum_zero_to_production::startup::app;
use std::net::SocketAddr;

#[tokio::test]
async fn health_check_success() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", app.address))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin@gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Should leverage another endpoint to check db state
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, expected_error) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "The API did not fail when {}",
            expected_error
        );
    }
}

struct TestApp {
    address: SocketAddr,
}

async fn spawn_app() -> Result<TestApp, Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });
    Ok(TestApp { address })
}
