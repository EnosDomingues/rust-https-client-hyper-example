use std::time::Instant;

use hyper::{Client, body::Buf};
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

    // Parse an `http::Uri`...
    let uri = "https://ftx.com/api/markets/BTC-PERP".parse()?;

    // Await the response...
    let resp = client.get(uri).await?;


    let body = hyper::body::aggregate(resp).await?;

    let data: SingleMarket = serde_json::from_reader(body.reader())?;
    println!("{:#?}", data);

    let duration = start.elapsed();
    println!("{:?}", duration);

    Ok(())
}