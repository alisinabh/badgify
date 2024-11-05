#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    marketh_rs::start_server("0.0.0.0", 8080).await;

    Ok(())
}
