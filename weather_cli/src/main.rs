use anyhow::Result;
use futures::future::join_all;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    weatherDesc: Vec<WeatherDesc>,
}

#[derive(Debug, Deserialize)]
struct WeatherDesc {
    value: String,
}

async fn fetch_weather(city: String) -> Result<()> {
    let url = format!("https://wttr.in/{city}?format=j1");

    let response: WeatherResponse = reqwest::get(&url)
        .await?
        .json()
        .await?;

    let current = response
        .current_condition
        .first();

    match current {
        Some(condition) => {
            let desc = condition
                .weatherDesc
                .first()
                .map(|d| d.value.as_str())
                .unwrap_or("Unknown");

            println!(
                "{}: {}°C {}",
                city,
                condition.temp_c,
                desc
            );
        }

        None => {
            println!("{city}: no weather data");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cities: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if cities.is_empty() {
        anyhow::bail!("please provide cities");
    }

    let tasks = cities
        .into_iter()
        .map(fetch_weather);

    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("error: {e}");
        }
    }

    Ok(())
}