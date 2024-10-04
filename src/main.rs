
use error_chain::error_chain;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json::Value;
use dotenv::dotenv;
use std::env;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        JsonError(serde_json::Error);
    }
}

const BASE_URL: &str = "https://rest.coinapi.io/v1/assets";

#[tokio::main]  
async fn main() -> Result<()> {

    dotenv().ok(); // Charge les variables d'environnement
    let api_key = env::var("API_KEY").expect("API_KEY not set");


    let client = Client::new();
    
    let res = client
        .get(BASE_URL)
        .header("X-CoinAPI-Key", api_key)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
        println!("The API call worked. Status: {}", res.status());
       // println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        let json_data:Value = serde_json::from_str(&body)?; // déserialisation du json

        if let Some(array) = json_data.as_array() {
            let frist_10 = &array[..std::cmp::min(10, array.len())];
            println!("Frist 10 elements: {:#?}", frist_10);
        } else {
            println!("The response is not an array.");
        }
    } else {
        println!("API call failed with status: {}", res.status());
    }

    Ok(())
}
