use average::WeightedMean;
use rust_decimal::{
    prelude::ToPrimitive,
    Decimal,
};
use serde::{Deserialize, Serialize};
use crate::api::shared::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceAdv {
    pub price: Decimal,
    pub tradable_quantity: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceAdvertiser {
    pub user_no: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PResponse {
    pub data: Vec<BinanceP2PResponseItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PResponseItem {
    pub adv: BinanceAdv,
    pub advertiser: BinanceAdvertiser,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PSearch {
    pub asset: String,
    pub fiat: String,
    pub page: u128,
    pub rows: u128,
    pub trade_type: String,
}

pub async fn get_price_from_binance_p2p(
    asset: &str,
    fiat: &str,
) -> Result<BinanceP2PResponse, Error> {
    let body = BinanceP2PSearch {
        asset: String::from(asset),
        fiat: String::from(fiat),
        page: 1u128,
        rows: 20u128,
        trade_type: String::from("BUY"),
    };
    let client = reqwest::Client::builder().build().unwrap();
    let res = client
        .post("https://p2p.binance.com/bapi/c2c/v2/friendly/c2c/adv/search")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(res) => match res.error_for_status() {
            Ok(res) => match &res.json::<BinanceP2PResponse>().await {
                //TODO: improve
                Ok(res) => Ok(BinanceP2PResponse {
                    data: res.data.clone(),
                }),
                Err(e) => return Err(Error::from(e)),
            },
            Err(e) => return Err(Error::from(&e)),
        },
        Err(e) => Err(Error::from(&e)),
    }
}

#[deprecated]
pub async fn get_binance_mean_p2p_price(asset: &str, fiat: &str) -> f64 {
    let mut binance_price = 0.0;
    let binance_response: Result<BinanceP2PResponse, Error> =
        get_price_from_binance_p2p(asset, fiat).await;
    match binance_response {
        Ok(res) => {
            let mut weigthed_mean = WeightedMean::new();
            res.data.iter().for_each(|item| {
                weigthed_mean.add(
                    item.adv.price.to_f64().unwrap(),
                    item.adv.tradable_quantity.to_f64().unwrap(),
                )
            });
            binance_price = weigthed_mean.mean();
        }
        Err(e) => println!("Error: {}", e),
    };
    binance_price
}