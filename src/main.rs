use core::fmt;

use average::{Mean, WeightedMean};
use models::{BinanceP2PSearch, CalypsoResponse};
use reqwest::Error as ReqwestError;
use rust_decimal::prelude::{ToPrimitive, Zero};

use crate::models::BinanceP2PResponse;
pub mod models;

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
        write!(f, "Error Huehuehueh")
    }
}

async fn get_price_from_binance_p2p(asset: &str, fiat: &str) -> Result<BinanceP2PResponse, Error> {
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

// Only supports ARS
async fn get_price_from_calypso(asset: &str) -> Result<CalypsoResponse, Error> {
    let url = format!("https://criptoya.com/api/calypso/{}/ars", asset);
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.error_for_status() {
            Ok(res) => match &res.json::<CalypsoResponse>().await {
                Ok(res) => Ok(CalypsoResponse {
                    ask: res.ask,
                    time: res.time,
                }),
                Err(e) => return Err(Error::from(e)),
            },
            Err(e) => return Err(Error::from(&e)),
        },
        Err(reqwest_error) => {
            return Err(Error::from(&reqwest_error));
        }
    }
}

#[tokio::main]
async fn main() {
    // Define vars
    let mut binance_price: f64 = 0f64;
    let mut calypso_price: f64 = 0f64;

    // Get price from Binance
    let binance_response = get_price_from_binance_p2p("USDT", "ARS").await;
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

    // Get price from Calypso
    let calypso_response = get_price_from_calypso("USDT").await;
    match calypso_response {
        Ok(res) => {
            calypso_price = res.ask.to_f64().unwrap_or(0f64);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Print results
    println!("Binance price {}", binance_price);
    println!("Calypso price {}", calypso_price);
    if binance_price == calypso_price && calypso_price.is_zero() {
        println!("Failed to fetch price from both sources");
    } else if !binance_price.is_zero() && !calypso_price.is_zero() {
        println!("Got price from both sources, calculate weigthed mean");
        let mut weigthed_mean_price = WeightedMean::new();
        weigthed_mean_price.add(binance_price, 3.0);
        weigthed_mean_price.add(calypso_price, 1.0);
        let mean_price: Mean = [binance_price, calypso_price].iter().collect();
        println!("Mean price {}", mean_price.mean());
        println!("Weigthed mean price {}", weigthed_mean_price.mean());
    } else {
        let arr = [binance_price, calypso_price];
        let price = arr.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        println!("Price is {}", &price);
        if binance_price.is_zero() {
            println!("Failed to get price from Binance");
        }
        if calypso_price.is_zero() {
            println!("Failed to get price from Calypso")
        }
    }
}
