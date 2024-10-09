use chrono::{NaiveDate, Duration, Utc};

use crate::rates_data_manager::get_and_manage_rates_data; 

mod api_config;
mod api_coin_service;
mod rates_data_manager;

/*
 * fonction principale
 *  exécute la gestion des taux de change en appelant `get_and_manage_rates_data`.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date_start = NaiveDate::from_ymd_opt(2024, 8, 1).unwrap();
    let date_end = Utc::now().date_naive() - Duration::days(1);
    let assets = "BTC/EUR";

    let rates = get_and_manage_rates_data(assets, date_start, date_end).await?;
    println!("Nombre de taux récupérés: {}", rates.len());

    Ok(())
}

