mod api_config;
mod api_coin_service;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api_coin_service::api_coin_service().await?;
    
    Ok(())
}