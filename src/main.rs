use core::fmt;

use average::{Mean, WeightedMean};
use models::{BinanceP2PSearch, BudaResponse, CalypsoResponse, MercadoBitcoinResponse};
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
        write!(f, "Error TODO")
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

async fn get_binance_mean_p2p_price(asset: &str, fiat: &str) -> f64 {
    let mut binance_price = 0.0;
    let binance_response = get_price_from_binance_p2p(asset, fiat).await;
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

// Only supporst ARS as fiat
async fn get_calypso_price(asset: &str) -> f64 {
    let mut calypso_price = 0.0;
    let calypso_response = get_price_from_calypso(asset).await;
    match calypso_response {
        Ok(res) => {
            calypso_price = res.ask.to_f64().unwrap_or(0f64);
        }
        Err(e) => println!("Error: {}", e),
    }
    calypso_price
}

// Only supports BRL
async fn get_price_from_mercado_bitcoin(asset: &str) -> f64 {
    let mut mercado_bitcoin_price = 0.0;
    let url = format!("https://www.mercadobitcoin.net/api/{}/ticker/", asset);
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.json::<MercadoBitcoinResponse>().await {
            Ok(mercado_bitcoin_response) => {
                mercado_bitcoin_price = mercado_bitcoin_response.ticker.get_price();
            }
            Err(e) => println!("Parsing Error: {:#?}", e),
        },
        Err(e) => {
            println!("Error: {:#?}", e)
        }
    }

    mercado_bitcoin_price
}

async fn get_buda_price(asset: &str, fiat: &str) -> f64 {
    let mut buda_price = 0.0;
    let url = format!(
        "https://www.buda.com/api/v2/markets/{}-{}/ticker.json",
        asset, fiat
    );
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.json::<BudaResponse>().await {
            Ok(buda_response) => buda_price = buda_response.ticker.get_last_price(),
            Err(e) => {
                println!("{:#?}", e);
            }
        },
        Err(e) => println!("{:#?}", e),
    }
    buda_price
}

#[tokio::main]
async fn main() {
    // Get USDT/ARS from BinanceP2P and Calypso
    let mut ars_mean_price = WeightedMean::new();
    let binance_price_ars: f64 = get_binance_mean_p2p_price("USDT", "ARS").await;
    let calypso_price_ars: f64 = get_calypso_price("USDT").await;
    ars_mean_price.add(binance_price_ars, 3.0);
    ars_mean_price.add(calypso_price_ars, 1.0);
    println!("USD/ARS: {}", ars_mean_price.mean());

    // Get USDT/BRL from BinanceP2P and USDC/BRL from MercadoBitcoin
    let binance_price_brl: f64 = get_binance_mean_p2p_price("USDT", "BRL").await;
    let mercado_bitcoin_price_brl = get_price_from_mercado_bitcoin("USDC").await;
    let brl_prices = &[binance_price_brl, mercado_bitcoin_price_brl];
    let brl_mean_price: Mean = brl_prices.iter().map(|x| *x).collect();
    println!("USD/BRL: {}", brl_mean_price.mean());

    // Get USDT/COP from BinanceP2P and USDC/COP from Buda
    let binance_price_cop: f64 = get_binance_mean_p2p_price("USDT", "COP").await;
    let buda_price_cop: f64 = get_buda_price("USDC", "COP").await;
    let mut cop_mean_price = WeightedMean::new();
    cop_mean_price.add(binance_price_cop, 3.0);
    cop_mean_price.add(buda_price_cop, 1.0);
    println!("USD/COP: {}", cop_mean_price.mean());

    // Print results
    println!("Binance price {}", binance_price_ars);
    println!("Calypso price {}", calypso_price_ars);

    // Extract to function and reuse for all fiat currencies
    if binance_price_ars == calypso_price_ars && calypso_price_ars.is_zero() {
        println!("Failed to fetch price from both sources");
    } else if !binance_price_ars.is_zero() && !calypso_price_ars.is_zero() {
        println!("Got price from both sources, calculate weigthed mean");
        let mut weigthed_mean_price = WeightedMean::new();
        weigthed_mean_price.add(binance_price_ars, 3.0);
        weigthed_mean_price.add(calypso_price_ars, 1.0);
        let mean_price: Mean = [binance_price_ars, calypso_price_ars].iter().collect();
        println!("Mean price {}", mean_price.mean());
        println!("Weigthed mean price {}", weigthed_mean_price.mean());
    } else {
        let arr = [binance_price_ars, calypso_price_ars];
        let price = arr.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        println!("Price is {}", &price);
        if binance_price_ars.is_zero() {
            println!("Failed to get price from Binance");
        }
        if calypso_price_ars.is_zero() {
            println!("Failed to get price from Calypso")
        }
    }
}
