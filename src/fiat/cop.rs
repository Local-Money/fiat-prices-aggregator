use serde::{Deserialize, Serialize};

use crate::shared::{get_avg_from_sources, get_binance_mean_p2p_price, PriceSource};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BudaResponse {
    pub ticker: BudaTicker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BudaTicker {
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

// Get USDT/COP from BinanceP2P and USDC/COP from Buda
pub async fn get_cop_price() -> f64 {
    let binance_price_cop: f64 = get_binance_mean_p2p_price("USDT", "COP").await;
    let buda_price_cop: f64 = get_buda_price("USDC", "COP").await;

    let price_source_a = PriceSource {
        price: binance_price_cop,
        fiat: String::from("COP"),
        source: String::from("BinanceP2P"),
        weight: 3.0,
    };
    let price_source_b = PriceSource {
        price: buda_price_cop,
        fiat: String::from("COP"),
        source: String::from("Buda"),
        weight: 1.0,
    };
    get_avg_from_sources(price_source_a, price_source_b)
}
