use axum_zero_to_production::configuration;
use axum_zero_to_production::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to load configuration");
    run(configuration).await;
    Ok(())
}
