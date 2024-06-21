use std::{env, process};
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ExchangeRates {
    base_code: String,
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // get the api key
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <amount> <from_currency> <to_currency>");
        process::exit(1);
    }

    // extract the inputs 
    let amount: f64 = args[1].trim().parse().expect("invalid amount");
    let from = &args[2].to_uppercase();
    let to = &args[3].to_uppercase();

    // make api call
    let response: ExchangeRates = reqwest::get(format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, from
    )).await?.json().await?;
    
    // do the calculations
    let converted_amount = amount * response.conversion_rates[to];
    println!("✨ {} {} is equals to {} {}", amount, response.base_code, converted_amount, to);

    Ok(())
}
