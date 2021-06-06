use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
use hyper::{Client, /* Body, Method, Request, */ Uri, body::Buf};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
extern crate hex;

type HmacSha256 = Hmac<Sha256>;

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

    /* let key = "7LBTqPT3g4vhpZ1mNoQN8nQPm-xmqPHvknnPUUR3"; */

    let secret = "xrTiHQj3XSul0-7LL-MUD9mqb3WOTiYws24bb0fh";

    let ts: String = 
        SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_millis().to_string();

    let signature_payload = format!("{}{}", ts, 
    "POST/api/orders{
        \"market\": \"BTC/USD\", 
        \"side\": \"buy\", 
        \"type\": \"market\", 
        \"size\": \"0.0001\", 
        \"externalReferralProgram\": \"oBot\", 
    }");

    let signature: String = format!("{}{}", secret, signature_payload);

    let mac = HmacSha256::new_from_slice(signature.as_bytes()).unwrap();

    let result = mac.finalize();

    let code_bytes = result.into_bytes();

    let encoded = hex::encode(&code_bytes);

    println!("{:#?}", encoded);

    /* let req = Request::builder()
    .method(Method::POST)
    .uri("https://ftx.com/api")
    .header("content-type", "application/json")
    .header("FTX-KEY", key)
    .header("FTX-TS", ts)
    .header("FTX-SIGN", signature)
    .body(Body::from(r#"{"library":"hyper"}"#))?; */
    
    let https = HttpsConnector::new();

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

    println!("{:#?} - {:#?}", btcusd_price.result.name, btcusd_price.result.bid);
    println!("{:#?} - {:#?}", ethusd_price.result.name, ethusd_price.result.bid);
    println!("{:#?} - {:#?}", ethbtc_price.result.name, ethbtc_price.result.bid);

    let duration = start.elapsed();
    println!("{:?}", duration);

    Ok(()) 
}