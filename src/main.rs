use average::WeightedMean;
use models::BinanceP2PSearch;
use rust_decimal::prelude::ToPrimitive;

use crate::models::BinanceP2PResponse;
pub mod models;

async fn binance_search() -> BinanceP2PResponse {
    let body = BinanceP2PSearch {
        asset: String::from("USDT"),
        fiat: String::from("ARS"),
        page: 1u128,
        rows: 20u128,
        trade_type: String::from("BUY"),
    };
    let client = reqwest::Client::builder().build().unwrap();
    let res = client
        .post("https://p2p.binance.com/bapi/c2c/v2/friendly/c2c/adv/search")
        .json(&body)
        .send()
        .await
        .unwrap();
    match res.error_for_status_ref() {
        Ok(_) => println!("Great success"),
        Err(e) => println!("Error: {:#?}", e),
    };
    res.json::<BinanceP2PResponse>().await.unwrap()
}

#[tokio::main]
async fn main() {
    let res = binance_search().await;
    let mut weigthed_mean = WeightedMean::new();
    res.data.iter().for_each(|item| {
        weigthed_mean.add(
            item.adv.price.to_f64().unwrap(),
            item.adv.tradable_quantity.to_f64().unwrap(),
        )
    });
    println!("Mean price {}", weigthed_mean.mean());
}
