use chrono::{NaiveDate, Duration, Utc};
use plotters::prelude::*;
use plotters::style::IntoFont;

use crate::rates_data_manager::get_and_manage_rates_data; 

mod api_config;
mod api_coin_service;
mod rates_data_manager;


/*
 * fonction pour afficher les taux de change sous forme de graphique dans une fenêtre
 */
fn plot_exchange_rates(dates: Vec<chrono::NaiveDate>, values: Vec<f64>, asset: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("output.png", (1280, 720)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(format!("Taux de Change: {}", asset), ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(dates[0]..dates[dates.len() - 1], 0.0..*values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        dates.iter().zip(values.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?
    .label("Valeur")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().background_style(&WHITE).border_style(&BLACK).draw()?;

    root_area.present()?;
    Ok(())
}

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
    println!("Number of rates recovered: {}", rates.len());

        // extraire les dates et les valeurs des taux
        let rates_dates: Vec<NaiveDate> = rates.iter().map(|r| NaiveDate::parse_from_str(&r.date, "%Y-%m-%d").unwrap()).collect();
        let rates_values: Vec<f64> = rates.iter().map(|r| r.value.clone()).collect();

    plot_exchange_rates(rates_dates, rates_values, assets)?;

    Ok(())
}

