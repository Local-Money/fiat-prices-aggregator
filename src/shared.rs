use average::WeightedMean;
use core::fmt;
use reqwest::Error as ReqwestError;
use rust_decimal::{
    prelude::{ToPrimitive, Zero},
    Decimal,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Request,
    Response,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<&ReqwestError> for Error {
    fn from(reqwest_error: &ReqwestError) -> Self {
        if reqwest_error.is_request() {
            Error::new(ErrorKind::Request)
        } else {
            Error::new(ErrorKind::Response)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write different msgs for different error types
        write!(f, "Error TODO")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub account: AccountInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountInfo {
    pub account_number: String,
    pub sequence: String,
}

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PriceSource {
    pub price: f64,
    pub fiat: String,
    pub source: String,
    pub weight: f64,
}

pub fn get_avg_from_sources(price_source_a: PriceSource, price_source_b: PriceSource) -> f64 {
    if price_source_a.price == price_source_b.price && price_source_b.price.is_zero() {
        println!(
            "Failed to fetch {} price from both sources.",
            price_source_a.fiat
        );
        return 0.0;
    } else if !price_source_a.price.is_zero() && !price_source_b.price.is_zero() {
        let mut mean_price = WeightedMean::new();
        mean_price.add(price_source_a.price, price_source_a.weight);
        mean_price.add(price_source_b.price, price_source_b.weight);
        return mean_price.mean();
    } else {
        let arr = [price_source_a.price, price_source_b.price];
        let price = arr.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        if price_source_a.price.is_zero() {
            println!(
                "Failed to get {} price from {}",
                price_source_a.fiat, price_source_a.source
            );
        }
        if price_source_b.price.is_zero() {
            println!(
                "Failed to get {} price from {}",
                price_source_b.fiat, price_source_b.source
            );
        }
        return *price;
    }
}
