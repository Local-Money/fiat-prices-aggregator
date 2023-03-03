use serde::{Deserialize, Serialize};
use crate::api::shared::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct MercadoBitcoinResponse {
    pub ticker: MercadoBitcoinTicker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct MercadoBitcoinTicker {
    pub last: String,
}

impl MercadoBitcoinTicker {
    fn get_price(&self) -> f64 {
        match self.last.parse::<f64>() {
            Ok(price) => price,
            Err(e) => {
                println!("{:#?}", e);
                0.0
            }
        }
    }
}

pub async fn get_price_from_mercado_bitcoin(asset: &str) -> Result<f64, Error> {
    let url = format!("https://www.mercadobitcoin.net/api/{}/ticker/", asset);
    match reqwest::get(url).await {
        Ok(res) => match res.json::<MercadoBitcoinResponse>().await {
            Ok(mercado_bitcoin_response) => Ok(mercado_bitcoin_response.ticker.get_price()),
            Err(e) => Err(Error::from(&e)),
        },
        Err(e) => Err(Error::from(&e))
    }
}