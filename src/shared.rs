use average::WeightedMean;
use rust_decimal::prelude::Zero;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub account: AccountInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountInfo {
    pub account_number: String,
    pub sequence: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PriceSource {
    pub price: f64,
    pub fiat: String,
    pub source: String,
    pub weight: f64,
}

#[deprecated]
pub fn get_avg_from_sources(price_source_a: PriceSource, price_source_b: PriceSource) -> f64 {
    if price_source_a.price == price_source_b.price && price_source_b.price.is_zero() {
        println!(
            "Failed to fetch {} price from both sources.",
            price_source_a.fiat
        );
        return 0.0;
    } else if !price_source_a.price.is_zero() && !price_source_b.price.is_zero() {
        let mut mean_price = WeightedMean::new();
        mean_price.add(price_source_a.price, price_source_a.weight);
        mean_price.add(price_source_b.price, price_source_b.weight);
        return mean_price.mean();
    } else {
        let arr = [price_source_a.price, price_source_b.price];
        let price = arr.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        if price_source_a.price.is_zero() {
            println!(
                "Failed to get {} price from {}",
                price_source_a.fiat, price_source_a.source
            );
        }
        if price_source_b.price.is_zero() {
            println!(
                "Failed to get {} price from {}",
                price_source_b.fiat, price_source_b.source
            );
        }
        return *price;
    }
}
