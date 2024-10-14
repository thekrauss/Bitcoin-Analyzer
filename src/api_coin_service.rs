use reqwest::{Client, StatusCode};
use serde::Deserialize;
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

/*
 * fonction `get_dates_intervals`
 *  divise une période de dates en plusieurs intervalles, chaque intervalle étant limité
 * par un nombre maximum de jours. Cela est nécessaire pour appeler une API qui a une limitation sur 
 * le nombre de jours que l'on peut interroger en une seule requête.
 */
pub fn get_dates_intervals(date_start: NaiveDate, date_end: NaiveDate, max_days: i64) -> Vec<(NaiveDate, NaiveDate)> {
    let mut intervals = Vec::new();
    let mut current_start = date_start;
    let mut diff_days = (date_end - date_start).num_days(); 

    while diff_days > 0 {
        // déterminer combien de jours ajouter à cet intervalle (max_days ou moins)
        let interval_length = std::cmp::min(max_days - 1, diff_days);
        let current_end = current_start + Duration::days(interval_length);
        intervals.push((current_start, current_end)); 
        current_start = current_end + Duration::days(1);
        diff_days -= interval_length + 1; 
    }

    intervals 
}

/*
 * fonction `api_coin_exchange_rates`
 * cette fonction appelle l'API pour récupérer les taux de change pour une période spécifique.
 * elle construit une URL avec les dates de début et de fin, puis désérialise la réponse JSON en
 * une liste de taux de change.
 */


pub async fn api_coin_exchange_rates(assets: &str, start: &str, end: &str) -> Result<Vec<ExchangeRate>, Box<dyn Error>> {
    let api_key = get_api_key();
    let client = Client::new();

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
        let body = res.text().await?;
        let rates: Vec<ExchangeRate> = serde_json::from_str(&body)?; 
        Ok(rates)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error in API call: {}", res.status()),
        )))
    }
}

/*
 * fonction `api_coin_exchange_rates_extended`
 * cette fonction permet d'appeler l'API pour des périodes plus longues que la limite imposée (par exemple, 100 jours).
 * elle utilise la fonction `get_dates_intervals` pour diviser une période longue en sous-intervalles,
 * puis fait des appels d'API pour chaque sous-intervalle et agrège les résultats.
 */
pub async fn api_coin_exchange_rates_extended(
    assets: &str,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<ExchangeRate>, Box<dyn Error>> {
    let intervals = get_dates_intervals(start, end, 100); 
    let mut all_rates = Vec::new();

    // appel API pour chaque intervalle
    for (start_interval, end_interval) in intervals {
        let start_str = start_interval.format("%Y-%m-%d").to_string();
        let end_str = end_interval.format("%Y-%m-%d").to_string();

        match api_coin_exchange_rates(assets, &start_str, &end_str).await {
            Ok(rates) => all_rates.extend(rates), 
            Err(e) => println!("Error for interval {} - {} : {}", start_str, end_str, e),
        }
    }

    Ok(all_rates) // retourner tous les taux pour la période totale
}
