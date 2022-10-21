use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceAdv {
    pub price: Decimal,
    pub tradable_quantity: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceAdvertiser {
    pub user_no: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PResponse {
    pub data: Vec<BinanceP2PResponseItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PResponseItem {
    pub adv: BinanceAdv,
    pub advertiser: BinanceAdvertiser,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinanceP2PSearch {
    pub asset: String,
    pub fiat: String,
    pub page: u128,
    pub rows: u128,
    pub trade_type: String,
}

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalypsoResponse {
    pub ask: Decimal,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MercadoBitcoinResponse {
    pub ticker: MercadoBitcoinTicker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MercadoBitcoinTicker {
    pub last: String,
}

impl MercadoBitcoinTicker {
    pub fn get_price(&self) -> f64 {
        match self.last.parse::<f64>() {
            Ok(price) => price,
            Err(e) => {
                println!("{:#?}", e);
                0.0
            }
        }
    }
}
