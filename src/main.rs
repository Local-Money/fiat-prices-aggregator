pub mod fiat;
pub mod shared;
use crate::fiat::{ars::get_ars_price, brl::get_brl_price, cop::get_cop_price};

#[tokio::main]
async fn main() {
    let ars_mean_price = get_ars_price().await;
    println!("USD/ARS: {}", ars_mean_price);

    let brl_mean_price = get_brl_price().await;
    println!("USD/BRL: {}", brl_mean_price);

    let cop_mean_price = get_cop_price().await;
    println!("USD/COP: {}", cop_mean_price);
}
