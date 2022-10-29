extern crate dotenv;

pub mod fiat;
pub mod shared;
use std::env::var;

use crate::fiat::{ars::get_ars_price, brl::get_brl_price, cop::get_cop_price};
use bip39::Mnemonic;
use cosmrs::{
    bip32,
    cosmwasm::MsgExecuteContract,
    crypto::secp256k1,
    rpc::HttpClient,
    tendermint::chain::Id,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    AccountId, Coin,
};
use cosmwasm_std::Uint128;
use dotenv::dotenv;
use localterra_protocol::{
    currencies::FiatCurrency,
    offer::{CurrencyPrice, ExecuteMsg::UpdatePrice},
};
use shared::AccountResponse;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let ars = get_ars_price().await;
    let brl = get_brl_price().await;
    let cop = get_cop_price().await;
    let prices = vec![
        (ars, FiatCurrency::ARS),
        (brl, FiatCurrency::BRL),
        (cop, FiatCurrency::COP),
    ];

    // Derivate Wallet from Seed
    let path = "m/44'/118'/0'/0/0"
        .parse::<bip32::DerivationPath>()
        .unwrap();
    let seed_words = var("ADMIN_SEED").unwrap();
    let mnemonic = Mnemonic::parse_normalized(&seed_words).unwrap();
    let seed = mnemonic.to_seed("");
    let sender_priv_key = secp256k1::SigningKey::derive_from_path(seed, &path).unwrap();
    let sender_pub_key = sender_priv_key.public_key();
    let sender_addr = sender_pub_key
        .account_id(var("ADDR_PREFIX").unwrap().as_str())
        .unwrap();

    // Fetch Account details, we need the account sequence number
    let account_url = format!(
        "{}cosmos/auth/v1beta1/accounts/{}",
        var("LCD").unwrap(),
        sender_addr.to_string()
    );
    let account_res = reqwest::get(account_url).await.unwrap();
    let account_data = account_res.json::<AccountResponse>().await.unwrap();
    println!("Account sequence is {}", account_data.account.sequence);

    // Send Tx to Contract
    let contract_addr = var("OFFER_ADDR").unwrap().parse::<AccountId>().unwrap();
    let mut tx_body_builder = tx::BodyBuilder::new();

    prices.iter().for_each(|price_fiat| {
        // TODO: although most currencies have 2 decimals,
        // some currencies like JPY have 3 and some exotic currencies have zero.
        let usd_price = Uint128::from((price_fiat.0 * 100.0).round() as u64);
        let fiat_currency = price_fiat.1.clone();
        let update_price_msg = UpdatePrice(CurrencyPrice {
            currency: fiat_currency,
            usd_price,
            updated_at: 0,
        });
        let json_msg = serde_json::to_string(&update_price_msg).unwrap();
        let execute_msg = MsgExecuteContract {
            sender: sender_addr.clone(),
            contract: contract_addr.clone(),
            msg: json_msg.into_bytes(),
            funds: vec![],
        };
        tx_body_builder.msg(execute_msg.into_any().unwrap());
    });

    let signer_info = SignerInfo::single_direct(
        Some(sender_pub_key.clone()),
        account_data
            .account
            .sequence
            .clone()
            .parse::<i64>()
            .unwrap() as u64,
    );
    let gas_amount = Coin {
        amount: 250_000u128,
        denom: "ukuji".parse().unwrap(),
    };
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(gas_amount, 1_000_000u64));
    let tx_body = tx_body_builder.finish();
    let account_number = account_data
        .account
        .account_number
        .clone()
        .parse::<i64>()
        .unwrap() as u64;
    let chain_id = var("CHAIN_ID").unwrap().parse::<Id>().unwrap();
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();
    let tx_signed = sign_doc.sign(&sender_priv_key).unwrap();
    let rpc_url = var("RPC").unwrap();
    let client = HttpClient::new(rpc_url.as_str()).unwrap();
    let res = tx_signed.broadcast_commit(&client).await.unwrap();
    println!("{}", res.deliver_tx.info.to_string());
    println!("res: {:#?}", res);
}
