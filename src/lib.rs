use axum::{Router, http::StatusCode, routing::get};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

pub async fn run() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
