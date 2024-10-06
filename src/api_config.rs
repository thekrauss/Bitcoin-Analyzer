use dotenv::dotenv;
use std::env;


pub fn get_api_key() -> String {
    dotenv().ok();       // charge le fichier .env
    env::var("API_KEY").expect("API_KEY not set") //récupère la clé API
}

pub const BASE_URL: &str = "https://rest.coinapi.io/v1/assets";