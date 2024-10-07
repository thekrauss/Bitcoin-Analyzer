use chrono::{Duration, Utc};
use api_coin_service::api_coin_exchange_rates;
use serde_json::json;
use std::fs::File;
use std::io::Write;

mod api_config;
mod api_coin_service;

// fonction pour sauvegarder les données JSON dans un fichier
fn save_json_data_to_file(filename: &str, json_data: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

// fonction pour formater les taux de change en JSON
fn get_json_rates(rates_data: &Vec<api_coin_service::ExchangeRate>) -> String{
    let rates_json: Vec<_> = rates_data.iter().map(|r|{
        json!({
            "date": r.time_period_start[..10].to_string(),
            "value": r.rate_close
        })
    }).collect();

    serde_json::to_string(&rates_json).unwrap()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // calcule la date d'aujourd'hui et la date de début (10 jours avant)
    let date_today = Utc::today();
    let date_start = date_today - Duration::days(10);

    let date_today_str = date_today.format("%Y-%m-%d").to_string();
    let date_start_str = date_start.format("%Y-%m-%d").to_string();

    println!("Start day: {}", date_start_str);
    println!("End day: {}", date_today_str);

        // défini les actifs pour l'API
    let assets = "BTC/EUR";
    
    // appel de l'API pour récupérer les taux de change
    let rates = api_coin_exchange_rates(&assets, &date_start_str, &date_today_str).await?;
    
    // vérification si des taux de change ont été reçus
    if !rates.is_empty(){

        let json = get_json_rates(&rates);

        let filename = assets.replace("/", "_") + ".json";

        save_json_data_to_file(&filename, &json);

        println!("{:}", json);
        println!("{}, nombre de cours: {}", assets, rates.len());
        
        for rate in &rates {
            println!("{} : {}", &rate.time_period_start[..10], rate.rate_close);
        }
    } else {
        println!("No exchange rate received.")
    }
    
    Ok(())
}