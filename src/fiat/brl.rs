use crate::api::mercado_bitcoin::get_price_from_mercado_bitcoin;
use crate::api::shared::PriceValue;

// return USDC/BRL from MercadoBitcoin
pub async fn get_brl_price() -> f64 {
    let mercado_bitcoin_price_brl = get_price_from_mercado_bitcoin("USDC").await;
    mercado_bitcoin_price_brl.value_or_log("get_price_from_mercado_bitcoin")
}

#[tokio::test]
async fn test_get_price_from_mercado_bitcoin_usdc_brl() {
    let price_source = get_price_from_mercado_bitcoin("USDC").await;
    assert!(price_source.is_ok());
    assert!(price_source.value_or_log("get_price_from_mercado_bitcoin").ne(&0f64));
}