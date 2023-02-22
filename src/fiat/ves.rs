use crate::shared::{get_avg_from_sources, get_binance_mean_p2p_price, PriceSource};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
struct Currency {
    USD: Price,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
struct Price {
    VES: f64,
}

async fn get_yadio_price() -> f64 {
    let mut price = 0.0;
    let url = "https://api.yadio.io/exrates/usd";
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => match res.json::<Currency>().await {
            Ok(currency) => price = currency.USD.VES,
            Err(e) => {
                println!("{:#?}", e);
            }
        },
        Err(e) => println!("{:#?}", e),
    }
    price
}

pub async fn get_ves_price() -> f64 {
    let binance_price: f64 = get_binance_mean_p2p_price("USDT", "VES").await;
    let yadio_price: f64 = get_yadio_price().await;

    let price_source_a = PriceSource {
        price: binance_price,
        fiat: String::from("VES"),
        source: String::from("BinanceP2P"),
        weight: 3.0,
    };
    let price_source_b = PriceSource {
        price: yadio_price,
        fiat: String::from("VES"),
        source: String::from("Yadio"),
        weight: 1.0,
    };
    get_avg_from_sources(price_source_a, price_source_b)
}
