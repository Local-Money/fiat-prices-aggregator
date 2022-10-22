use crate::shared::{get_avg_from_sources, get_binance_mean_p2p_price, PriceSource};
use serde::{Deserialize, Serialize};

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

// Get USDT/BRL from BinanceP2P and USDC/BRL from MercadoBitcoin
pub async fn get_brl_price() -> f64 {
    let binance_price_brl: f64 = get_binance_mean_p2p_price("USDT", "BRL").await;
    let mercado_bitcoin_price_brl = get_price_from_mercado_bitcoin("USDC").await;

    let price_source_a = PriceSource {
        price: binance_price_brl,
        fiat: String::from("BRL"),
        source: String::from("BinanceP2P"),
        weight: 1.0,
    };
    let price_source_b = PriceSource {
        price: mercado_bitcoin_price_brl,
        fiat: String::from("BRL"),
        source: String::from("MercadoBitcoin"),
        weight: 1.0,
    };
    get_avg_from_sources(price_source_a, price_source_b)
}
