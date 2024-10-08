use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tokio::time::Interval;
use std::error::Error;
use chrono::{NaiveDate, Duration};
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


pub fn get_dates_intervals(date_start: NaiveDate, date_end: NaiveDate, max_days: i64) -> Vec<(NaiveDate, NaiveDate)> {
    let mut intervals = Vec::new();
    let mut current_start = date_start;
    let mut diff_days = (date_end - date_start).num_days(); // calcule la différence en jours

    // tant qu'il reste des jours à diviser
    while diff_days > 0 {
        // déterminer combien de jours ajouter à cet intervalle (max_days ou moins)
        let interval_length = std::cmp::min(max_days - 1, diff_days);
        let current_end = current_start + Duration::days(interval_length);
        intervals.push((current_start, current_end)); // ajouter l'intervalle à la liste
        current_start = current_end + Duration::days(1); // déplacer le début du prochain intervalle
        diff_days -= interval_length + 1; // réduire le nombre de jours restants
    }

    intervals // retourner la liste des intervalles
}


// appeler l'API pour les taux de change d'une période donnée
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

//  appeler l'API pour plusieurs intervalles de dates
pub async fn api_coin_exchange_rates_extended(
    assets: &str,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<ExchangeRate>, Box<dyn Error>> {
    let intervals = get_dates_intervals(start, end, 100); // Diviser la période en intervalles de 100 jours
    let mut all_rates = Vec::new();

    // Appel API pour chaque intervalle
    for (start_interval, end_interval) in intervals {
        let start_str = start_interval.format("%Y-%m-%d").to_string();
        let end_str = end_interval.format("%Y-%m-%d").to_string();

        match api_coin_exchange_rates(assets, &start_str, &end_str).await {
            Ok(rates) => all_rates.extend(rates), // Ajouter les taux récupérés à la liste complète
            Err(e) => println!("Error for interval {} - {} : {}", start_str, end_str, e),
        }
    }

    Ok(all_rates) // Retourner tous les taux pour la période totale
}
