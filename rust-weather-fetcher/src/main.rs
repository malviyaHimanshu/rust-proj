use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    // get location from arguments
    let location = env::args().nth(1).expect("Location not specified");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        location, api_key
    );

    // make http get request
    let response: WeatherResponse = reqwest::get(&url).await?.json().await?;

    // display all the information
    println!("\n~ Weather in {} ~", response.name);
    println!("Temperature: {}°C", response.main.temp);
    println!("Feels like: {}°C", response.main.feels_like);
    println!("Min Temperature: {}°C", response.main.temp_min);
    println!("Max Temperature: {}°C", response.main.temp_max);
    println!("Pressure: {} hPa", response.main.pressure);
    println!("Humidity: {}%", response.main.humidity);

    Ok(())
}
