/*
 * Importation des bibliothèques et modules nécessaires.
 * - chrono : Pour manipuler les dates.
 * - serde : Pour la sérialisation et désérialisation JSON.
 * - std::fs et std::io : Pour lire et écrire des fichiers.
 * - api_coin_service : Module interne qui contient les fonctions d'appel API.
 */
use chrono::{NaiveDate, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::api_coin_service::{api_coin_exchange_rates_extended, get_dates_intervals};

mod api_config;
mod api_coin_service;

/*
 * Structure `Rate`
 * Cette structure représente un taux de change avec une date et une valeur associée.
 * Elle est dérivée avec `Serialize` et `Deserialize` pour permettre la sérialisation et
 * désérialisation en JSON.
 */
#[derive(Debug, Serialize, Deserialize)]
struct Rate {
    date: String,
    value: f64, 
}

/*
 * Fonction `get_json_rates`
 * Cette fonction convertit une liste de taux de change (`Vec<Rate>`) en une chaîne de caractères au format JSON.
 */
fn get_json_rates(rates_data: &Vec<Rate>) -> String {
    serde_json::to_string(&rates_data).unwrap()
}

/*
 * Fonction `load_json_data_from_file`
 * Elle charge un fichier JSON à partir du disque et désérialise son contenu en une liste de `Rate`.
 * Si le fichier n'est pas trouvé, elle déclenche une erreur.
 */
fn load_json_data_from_file(filename: &str) -> Vec<Rate> {
    let mut file = File::open(filename).expect("File no found");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Error to read file");
    serde_json::from_str(&json_data).expect("Error while deserializing JSON")
}

/*
 * Fonction `save_json_data_to_file`
 * Cette fonction prend une chaîne de caractères JSON et l'écrit dans un fichier.
 * Si le fichier n'existe pas, il est créé. Sinon, son contenu est remplacé.
 */
fn save_json_data_to_file(filename: &str, json_data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(json_data.as_bytes()).expect("Error writing file");
}

/*
 * Fonction principale asynchrone
 * - Elle calcule la date d'aujourd'hui et la date de début (10 jours avant aujourd'hui).
 * - Elle charge les données JSON si elles existent déjà, sinon elle fait un appel à l'API pour récupérer les données.
 * - Elle sauvegarde les nouvelles données JSON récupérées dans un fichier.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Calcule la date d'aujourd'hui et la date de début (10 jours avant aujourd'hui)
    let date_end: chrono::DateTime<Utc> = Utc::now(); // Date actuelle
    let date_end_str = date_end.format("%Y-%m-%d").to_string(); // Convertir la date actuelle en chaîne formatée

    let date_start = date_end - Duration::days(10); // Calculer la date de début (10 jours avant aujourd'hui)
    let date_start_str = date_start.format("%Y-%m-%d").to_string(); // Convertir en chaîne

    // Définir l'actif (ici, Bitcoin en Euro)
    let assets = "BTC/EUR";
    let data_filename = assets.replace("/", "_") + ".json"; // Créer un nom de fichier basé sur l'actif (par exemple "BTC_EUR.json")

    // Charger les données JSON à partir du fichier si le fichier existe
    let mut rates: Vec<Rate> = Vec::new();
    if Path::new(&data_filename).exists() {
        rates = load_json_data_from_file(&data_filename); // Charger les données JSON existantes
    }

    // Si des données existent déjà, afficher les dates de début et de fin
    if !rates.is_empty() {
        let saved_data_date_start_str = &rates[0].date;
        let saved_data_date_end_str = &rates[rates.len() - 1].date;
        println!("Saved data : start {}, end {}", saved_data_date_start_str, saved_data_date_end_str);
    } else {
        println!("No saved data found.");
    }

    // Conversion des chaînes de caractères en `NaiveDate`
    let date_start = NaiveDate::parse_from_str(&date_start_str, "%Y-%m-%d").expect("Invalid start date");
    let date_end = NaiveDate::parse_from_str(&date_end_str, "%Y-%m-%d").expect("Invalid end date");

    // Appel API pour récupérer les taux de change sur une période en utilisant des intervalles
    let intervals = get_dates_intervals(date_start, date_end, 100); // Diviser la période en intervalles de 100 jours
    for (start, end) in intervals {
        println!("Intervalles: {} - {}", start, end);

        // Appeler l'API pour chaque intervalle
        let rates_data = api_coin_exchange_rates_extended(assets, start, end).await?;
        for rate in rates_data {
            rates.push(Rate { 
                date: rate.time_period_start[..10].to_string(), // Extraire uniquement la date (format YYYY-MM-DD)
                value: rate.rate_close, // Taux de clôture
            });
        }
    }

    // Sauvegarder les données JSON dans un fichier
    let json_data = get_json_rates(&rates); // Convertir les taux de change en JSON
    save_json_data_to_file(&data_filename, &json_data); // Sauvegarder dans un fichier

    println!("{:?}", rates); // Afficher les taux de change récupérés
    println!("Data saved in {}", data_filename); // Confirmer la sauvegarde

    Ok(())
}
