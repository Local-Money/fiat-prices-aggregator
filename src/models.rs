use cosmwasm_std::Decimal;
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
