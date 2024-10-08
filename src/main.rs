use chrono::{NaiveDate, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::api_coin_service::{api_coin_exchange_rates_extended, get_dates_intervals};




mod api_config;
mod api_coin_service;

#[derive(Debug, Serialize, Deserialize)]
struct Rate {
    date: String,
    value: f64, 
}



// fonction pour formater les taux de change en JSON
fn get_json_rates(rates_data: &Vec<Rate>) -> String {
    serde_json::to_string(&rates_data).unwrap()
}


// charger des données JSON à partir d'un fichier
fn load_json_data_from_file(filename: &str) -> Vec<Rate> {
    let mut file = File::open(filename).expect("File no found");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Error reading file");
    serde_json::from_str(&json_data).expect("Error while deserializing JSON")
}

    // sauvegarder les données JSON dans un fichier
fn save_json_data_to_file(filename: &str, json_data: &str) {
    let mut file = File::create(filename).expect("Impossible de créer le fichier");
    file.write_all(json_data.as_bytes()).expect("Erreur lors de l'écriture du fichier");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // calcule la date d'aujourd'hui et la date de début (10 jours avant)
    let date_end: chrono::DateTime<Utc> = Utc::now();
    let date_end_str = date_end.format("%Y-%m-%d").to_string();


    let date_start = date_end - Duration::days(10);
    let date_start_str = date_start.format("%Y-%m-%d").to_string();


        // défini les actifs pour l'API
    let assets = "BTC/EUR";
    let data_filename = assets.replace("/", "_") + ".json";

        // Charger les données JSON depuis le fichier si le fichier existe
    let mut rates : Vec<Rate> = Vec::new();
    if Path::new(&data_filename).exists(){
        rates = load_json_data_from_file(&data_filename);
    }
        // Si des données sont déjà présentes dans le fichier
    if !rates.is_empty(){
        let saved_data_date_start_str = &rates[0].date;
        let saved_data_date_end_str = &rates[rates.len() -1].date;

        println!("Saved data : start {}, end {}", saved_data_date_start_str, saved_data_date_end_str);
    } else {
        println!("No saved data found.");
    }


     // Conversion des chaînes en NaiveDate avec parse_from_str
    let date_start = NaiveDate::parse_from_str(&date_start_str, "%Y-%m-%d").expect("Date start invalid");
    let date_end = NaiveDate::parse_from_str(&date_end_str, "%Y-%m-%d").expect("Date end invalid");

        // Appel API avec intervalles
        let intervals = get_dates_intervals(date_start, date_end, 100);
        for (start, end) in intervals  {
        println!("Intervals: {} - {}", start, end);

        let rates_data = api_coin_exchange_rates_extended(assets, date_start, date_end).await?;
        for rate in rates_data{
            rates.push(Rate { 
                date: rate.time_period_start[..10].to_string(),
                value: rate.rate_close,
            });
        }
    }

    let json_data =  get_json_rates(&rates);
    save_json_data_to_file(&data_filename, &json_data);

    println!("{:?}", rates);
    println!("Data saved in {}", data_filename);


        
    Ok(())
}