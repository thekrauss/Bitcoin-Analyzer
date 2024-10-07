use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::error::Error;
use crate::api_config::{get_api_key, BASE_URL};

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    pub time_period_start: String,
    pub time_period_end: String,
    pub time_open: String,
    pub time_close: String,
    pub rate_open: f64,
    pub rate_high: f64,
    pub rate_low: f64,
    pub rate_close: f64,
}

pub async fn api_coin_exchange_rates(assets: &str, start: &str, end: &str) -> Result<Vec<ExchangeRate>, Box<dyn Error>> {
    let api_key = get_api_key();
    let client = Client::new();

    // URL pour les taux de change Bitcoin en Euro sur une période donnée
    let url = format!(
        "{}v1/exchangerate/{}/history?period_id=1DAY&time_start={}&time_end={}",
        BASE_URL, assets, start, end
    );

    let res = client
        .get(&url)
        .header("X-CoinAPI-Key", api_key)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
        println!("The API call worked. Status: {}", res.status());
        let body = res.text().await?;
        let rates: Vec<ExchangeRate> = serde_json::from_str(&body)?; // Désérialiser le JSON en une liste de taux de change
        Ok(rates)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error in API call: {}", res.status()),
        )))
    }
}

/* 
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
*/