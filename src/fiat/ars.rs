use crate::shared::{get_avg_from_sources, get_binance_mean_p2p_price, Error, PriceSource};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalypsoResponse {
    pub ask: Decimal,
    pub time: u64,
}

async fn get_calypso_price(asset: &str) -> Result<f64, Error> {
    let calypso_price;
    let url = format!("https://criptoya.com/api/calypso/{}/ars", asset);
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.error_for_status() {
            Ok(res) => match &res.json::<CalypsoResponse>().await {
                Ok(res) => calypso_price = res.ask.to_f64().unwrap_or(0.0),
                Err(e) => return Err(Error::from(e)),
            },
            Err(e) => return Err(Error::from(&e)),
        },
        Err(reqwest_error) => {
            return Err(Error::from(&reqwest_error));
        }
    }
    Ok(calypso_price)
}

// Get USDT/ARS from BinanceP2P and USDT/ARS from Calypso (using CryptoYa API).
// Returns the weigthed avg of 3:1 Binace:Calypso if both prices are returned,
// return only one
pub async fn get_ars_price() -> f64 {
    let binance_price_ars: f64 = get_binance_mean_p2p_price("USDT", "ARS").await;
    let calypso_price_ars: f64 = get_calypso_price("USDT").await.unwrap_or(0f64);
    let price_source_a = PriceSource {
        price: binance_price_ars,
        fiat: String::from("ARS"),
        source: String::from("BinanceP2P"),
        weight: 3.0,
    };
    let price_source_b = PriceSource {
        price: calypso_price_ars,
        fiat: String::from("ARS"),
        source: String::from("Calypso"),
        weight: 1.0,
    };
    get_avg_from_sources(price_source_a, price_source_b)
}
