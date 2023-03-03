use crate::api::buda::get_buda_price;
use crate::api::shared::PriceValue;

// retrun USDC/COP from Buda
pub async fn get_clp_price() -> f64 {
    let buda_price_cop = get_buda_price("USDC", "CLP").await;
    buda_price_cop.value_or_log("get_buda_price")
}


#[tokio::test]
async fn test_get_buda_price_usdc_clp() {
    let price_source = get_buda_price("USDC", "CLP").await;
    assert!(price_source.is_ok());
    assert!(price_source.value_or_log("get_buda_price").ne(&0f64));
}