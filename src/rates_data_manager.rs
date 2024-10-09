use crate::api_coin_service::api_coin_exchange_rates_extended;
use chrono::{NaiveDate, Duration};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rate {
    pub date: String,
    pub value: f64,
}

/*
 * fonction `get_json_rates`
 * cette fonction convertit une liste de taux de change (`Vec<Rate>`) en une chaîne de caractères au format JSON.
 */
fn get_json_rates(rates_data: &Vec<Rate>) -> String {
    serde_json::to_string(&rates_data).unwrap()
}

/*
 * fonction `load_json_data_from_file`
 * elle charge un fichier JSON à partir du disque et désérialise son contenu en une liste de `Rate`.
 * si le fichier n'est pas trouvé, elle déclenche une erreur.
 */
fn load_json_data_from_file(filename: &str) -> Vec<Rate> {
    let mut file = File::open(filename).expect("File no found");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Error to read file");
    serde_json::from_str(&json_data).expect("Error while deserializing JSON")
}

/*
 * fonction `save_json_data_to_file`
 * cette fonction prend une chaîne de caractères JSON et l'écrit dans un fichier.
 * si le fichier n'existe pas, il est créé. Sinon, son contenu est remplacé.
 */
fn save_json_data_to_file(filename: &str, json_data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(json_data.as_bytes()).expect("Error writing file");
}


/*
 * fonction `convert_exchange_rates_to_date_value_format`
 * cette fonction convertit les taux de change bruts en un format simplifié `{ date, value }`.
 */
fn convert_exchange_rates_to_date_value_format(rates_data: &Vec<crate::api_coin_service::ExchangeRate>) -> Vec<Rate> {
    rates_data.iter().map(|r| Rate {
        date: r.time_period_start[..10].to_string(), 
        value: r.rate_close, // taux de clôture
    }).collect()
}

pub async fn get_and_manage_rates_data(assets: &str, date_start: NaiveDate, date_end: NaiveDate) -> Result<Vec<Rate>, Box<dyn std::error::Error>> {
    let data_filename = assets.replace("/", "_") + ".json"; 

    //let mut rates: Vec<Rate> = Vec::new();

    // Charger les données JSON à partir du fichier si le fichier existe
    let mut rates: Vec<Rate> = Vec::new();
    if Path::new(&data_filename).exists() {
        rates = load_json_data_from_file(&data_filename); 
    }

    if !rates.is_empty(){
        let saved_data_date_start_str = &rates[0].date;
        let saved_data_date_end_str = &rates[rates.len() - 1].date;

        println!("Le file JSON exist");
        println!("  saved_data_date_start_str: {}", saved_data_date_start_str);
        println!("  saved_data_date_end_str: {}", saved_data_date_end_str);

        // conversion des dates sauvegardées en `NaiveDate`
        let saved_data_date_start = NaiveDate::parse_from_str(saved_data_date_start_str, "%Y-%m-%d").expect("Date de début invalide");
        let saved_data_date_end = NaiveDate::parse_from_str(saved_data_date_end_str, "%Y-%m-%d").expect("Date de fin invalide");

        // vérifie s'il y a des jours manquants avant la période sauvegardée
        let nb_days_start = (saved_data_date_start - date_start).num_days();
        if nb_days_start > 0 {
            println!("adding rates before : {} - {}", date_start, saved_data_date_start - Duration::days(1));
            let rates_start = api_coin_exchange_rates_extended(assets, date_start, saved_data_date_start - Duration::days(1)).await?;
            let rates_start_date_value = convert_exchange_rates_to_date_value_format(&rates_start);
            rates = [rates_start_date_value, rates].concat(); // Ajouter les nouvelles données
        }

        // vérifie s'il y a des jours manquants après la période sauvegardée
        let nb_days_end = (date_end - saved_data_date_end).num_days();
        if nb_days_end > 0 {
            println!("add data after : {} - {}", saved_data_date_end + Duration::days(1), date_end);
            let rates_end = api_coin_exchange_rates_extended(assets, saved_data_date_end + Duration::days(1), date_end).await?;
            let rates_end_date_value = convert_exchange_rates_to_date_value_format(&rates_end);
            rates.extend(rates_end_date_value); // ajouter les nouvelles données

        } 
                // sauvegarder les données mises à jour dans le fichier JSON après avoir ajouté les nouvelles données
        let json_data = get_json_rates(&rates);
        save_json_data_to_file(&data_filename, &json_data);

    } else {
        println!("no data saved. Full recovery.");
        let rates_api = api_coin_exchange_rates_extended(assets, date_start, date_end).await?;
        rates = convert_exchange_rates_to_date_value_format(&rates_api);
        let json_data = get_json_rates(&rates);
        save_json_data_to_file(&data_filename, &json_data);
    }

    Ok(rates)

}