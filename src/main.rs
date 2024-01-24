use reqwest;
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() {
    match fetch_bitcoin_price().await {
        Ok(price) => println!("Current Bitcoin Price: {} USD", price),
        Err(e) => eprintln!("Error fetching current Bitcoin price: {:?}", e),
    }
}

async fn fetch_bitcoin_price() -> Result<String, reqwest::Error> {
    let url = "https://api.coindesk.com/v1/bpi/currentprice/USD.json";
    let response = reqwest::get(url).await?.json::<Value>().await?;

    // Extract the Bitcoin price
    Ok(response["bpi"]["USD"]["rate"].as_str().unwrap_or_default().to_string())
}

