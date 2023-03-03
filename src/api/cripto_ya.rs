use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use crate::api::shared::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct CriptoYaResponse {
    pub ask: Decimal,
    pub time: u64,
}

/***
 */
pub async fn get_crypto_ya_price(market: &str, asset: &str, fiat: &str) -> Result<f64, Error> {
    let url = format!("https://criptoya.com/api/{}/{}/{}", market, asset, fiat);
    match reqwest::get(url).await {
        Ok(res) => match res.error_for_status() {
            Ok(res) => match &res.json::<CriptoYaResponse>().await {
                Ok(res) => Ok(res.ask.to_f64().unwrap_or(0.0)),
                Err(e) => return Err(Error::from(e)),
            },
            Err(e) => return Err(Error::from(&e)),
        },
        Err(e) => Err(Error::from(&e))
    }
}

pub async fn get_calypso_price(asset: &str, fiat: &str) -> Result<f64, Error> {
    get_crypto_ya_price("calypso", asset, fiat).await
}

pub async fn get_bitso_price(asset: &str, fiat: &str) -> Result<f64, Error> {
    get_crypto_ya_price("bitso", asset, fiat).await
}