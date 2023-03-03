use crate::api::cripto_ya::get_calypso_price;
use crate::api::shared::PriceValue;

// returns USDT/USD from Calypso (using CryptoYa API).
pub async fn get_usd_price() -> f64 {
    let calypso_price_ars = get_calypso_price("USDT", "USD").await;
    calypso_price_ars.value_or_log("get_calypso_price")
}

#[tokio::test]
async fn test_get_calypso_price_usdt_ars() {
    let price_source = get_calypso_price("USDT", "USD").await;
    assert!(price_source.is_ok());
    assert!(price_source.value_or_log("get_calypso_price").ne(&0f64));
}