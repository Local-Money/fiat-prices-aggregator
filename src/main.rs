use models::BinanceP2PSearch;

use crate::models::BinanceP2PResponse;
pub mod models;

async fn binance_search() -> BinanceP2PResponse {
    let body = BinanceP2PSearch {
        asset: String::from("USDT"),
        fiat: String::from("ARS"),
        page: 1u128,
        rows: 10u128,
        trade_type: String::from("BUY"),
    };
    let client = reqwest::Client::builder()
        .build()
        .unwrap();
    let res = client
        .post("https://p2p.binance.com/bapi/c2c/v2/friendly/c2c/adv/search")
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:#?}", res);
    match res.error_for_status_ref() {
        Ok(_) => println!("Great success"),
        Err(e) => println!("Error: {:#?}", e),
    };
    res.json::<BinanceP2PResponse>().await.unwrap()
}

#[tokio::main]
async fn main() {
    let res = binance_search().await;
    println!("{:#?}", res);
}
