use serde::{Deserialize, Serialize};
use crate::api::shared::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
struct Currency {
    USD: Prices,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct Prices {
    pub ARS: f64,
    pub BRL: f64,
    pub CLP: f64,
    pub COP: f64,
    pub GBP: f64,
    pub MXN: f64,
    pub EUR: f64,
    pub VES: f64,
}

impl Default for Prices {
    fn default() -> Self {
        Prices {
            ARS: 0.0,
            BRL: 0.0,
            CLP: 0.0,
            COP: 0.0,
            GBP: 0.0,
            EUR: 0.0,
            MXN: 0.0,
            VES: 0.0,
        }
    }
}

pub async fn get_yadio_prices() -> Result<Prices, Error> {
    let url = "https://api.yadio.io/exrates/usd";
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.json::<Currency>().await {
            Ok(currency) => Ok(currency.USD),
            Err(e) => Err(Error::from(&e))
        },
        Err(e) => Err(Error::from(&e)),
    }
}


#[tokio::test]
async fn test_yadio_prices() {
    let price_source = get_yadio_prices().await;
    assert!(price_source.is_ok());
    let json_msg = serde_json::to_string(&price_source.unwrap()).unwrap();
    println!("prices: {}", json_msg)
}