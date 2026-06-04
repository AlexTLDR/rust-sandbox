use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=48.8828&longitude=9.8151&hourly=temperature_2m&current=temperature_2m&timezone=Europe%2FBerlin";
    // let response = reqwest::get(URL).await?;
    // println!("{:?}", response.text().await);

    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=48.8828&longitude=9.8151&current_weather=true";
    let response = reqwest::get(URL).await?;
    let weather: serde_json::Value = response.json().await?;

    // println!("Current temperature: {}°C", weather.current_weather.temperature);
    // println!("Current windspeed: {} km/h", weather.current_weather.windspeed);

    println!("{weather:#?}");

    Ok(())
}
