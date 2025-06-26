use axum_zero_to_production::app;
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
