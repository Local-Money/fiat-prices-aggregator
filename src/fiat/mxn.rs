use crate::api::cripto_ya::get_bitso_price;
use crate::api::shared::PriceValue;

// returns USDT/MXN from Calypso (using CryptoYa API).
pub async fn get_usd_price() -> f64 {
    let calypso_price_ars = get_bitso_price("USDC", "MXN").await;
    calypso_price_ars.value_or_log("get_bitso_price")
}

#[tokio::test]
async fn test_get_calypso_price_usdc_mxn() {
    let price_source = get_bitso_price("USDC", "MXN").await;
    assert!(price_source.is_ok());
    assert!(price_source.value_or_log("get_bitso_price").ne(&0f64));
}