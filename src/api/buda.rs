use serde::{Deserialize, Serialize};
use crate::api::shared::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct BudaResponse {
    pub ticker: BudaTicker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct BudaTicker {
    pub last_price: Vec<String>,
}

impl BudaTicker {
    pub fn get_last_price(&self) -> f64 {
        match self.last_price[0].parse::<f64>() {
            Ok(price) => price,
            Err(e) => {
                println!("{:#?}", e);
                0.0
            }
        }
    }
}

/***

 */
pub async fn get_buda_price(asset: &str, fiat: &str) -> Result<f64, Error> {
    let url = format!(
        "https://www.buda.com/api/v2/markets/{}-{}/ticker.json",
        asset, fiat
    );
    match reqwest::get(url).await {
        Ok(res) => match res.json::<BudaResponse>().await {
            Ok(buda_response) => Ok(buda_response.ticker.get_last_price()),
            Err(e) => Err(Error::from(&e))
        },
        Err(e) => Err(Error::from(&e)),
    }
}