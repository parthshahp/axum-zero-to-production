use axum_zero_to_production::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await;
    Ok(())
}
