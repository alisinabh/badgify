#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    badgify_rs::start_server("0.0.0.0", 8080).await;

    Ok(())
}
