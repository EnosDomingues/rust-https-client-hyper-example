use std::time::Instant;

use hyper::{Client, Uri, body::Buf};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SingleMarketResult {
    name: String,
    bid: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleMarket {
    success: bool,
    result: SingleMarketResult
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    
    // This is where we will setup our HTTP client requests.
    let https = HttpsConnector::new();

    // Still inside `async fn main`...
    let client = Client::builder()
    .build::<_, hyper::Body>(https);

    let btcusd_fut = async {
        let resp = client.get(Uri::from_static("https://ftx.com/api/markets/BTC/USD")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let ethusd_fut = async {
        let resp = client.get(Uri::from_static("https://ftx.com/api/markets/ETH/USD")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let ethbtc_fut = async {
        let resp = client.get(Uri::from_static("https://ftx.com/api/markets/ETH/BTC")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    // Wait on both them at the same time:
    let (btcusd, ethusd, ethbtc) = futures::try_join!(btcusd_fut, ethusd_fut, ethbtc_fut)?;

    let btcusd_price: SingleMarket = serde_json::from_reader(btcusd.reader())?;
    let ethusd_price: SingleMarket = serde_json::from_reader(ethusd.reader())?;
    let ethbtc_price: SingleMarket = serde_json::from_reader(ethbtc.reader())?;

    println!("{:#?}", btcusd_price.result.name);
    println!("{:#?}", ethusd_price.result.name);
    println!("{:#?}", ethbtc_price.result.name);


    let duration = start.elapsed();
    println!("{:?}", duration);

    Ok(()) 
}