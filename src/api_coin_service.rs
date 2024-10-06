use reqwest::{Client, StatusCode};
use serde_json::Value;
use std:: error::Error;
use crate::api_config::{get_api_key, BASE_URL};

pub async fn api_coin_service() -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key();
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