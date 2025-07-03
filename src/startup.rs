use axum::{
    Router,
    routing::{get, post},
};

use crate::routes;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
}

pub async fn run() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
