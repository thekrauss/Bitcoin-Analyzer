use error_chain::error_chain;
use reqwest::Client;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

const API_KEY: &str = "FE9D63C3-04B4-4994-96F9-FB84A83713C3";
const BASE_URL: &str = "https://rest.coinapi.io/v1/assets";

#[tokio::main]  
async fn main() -> Result<()> {
    let client = Client::new();
    
    let res = client
        .get(BASE_URL)
        .header("X-CoinAPI-Key", API_KEY)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
