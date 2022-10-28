extern crate dotenv;

pub mod fiat;
pub mod shared;
use base64::{self, Config};
use std::env::var;

use crate::fiat::{ars::get_ars_price, brl::get_brl_price, cop::get_cop_price};
use bip39::Mnemonic;
use cosmrs::{
    bank::MsgSend,
    bip32,
    cosmwasm::MsgExecuteContract,
    crypto::secp256k1,
    rpc::{Client, HttpClient},
    tendermint::{abci::Transaction, chain::Id},
    tx::{self, Fee, Msg, SignDoc, SignerInfo, Tx},
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
    let ars_mean_price = get_ars_price().await;
    let brl_mean_price = get_brl_price().await;
    let cop_mean_price = get_cop_price().await;
    println!("ARS: {}", ars_mean_price);
    println!("BRL: {}", brl_mean_price);
    println!("COP: {}", cop_mean_price);

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
        "http://localkujira.localmoney.io:1317/cosmos/auth/v1beta1/accounts/{}",
        sender_addr.to_string()
    );
    let account_res = reqwest::get(account_url).await.unwrap();
    let account_data = account_res.json::<AccountResponse>().await.unwrap();
    println!("Account sequence is {}", account_data.account.sequence);

    let contract_addr = var("OFFER_ADDR").unwrap().parse::<AccountId>().unwrap();
    let fiat_currency = FiatCurrency::ARS;
    let usd_price = Uint128::from(ars_mean_price.round() as u64);
    let update_price_msg = UpdatePrice(CurrencyPrice {
        currency: fiat_currency,
        usd_price,
        updated_at: 0,
    });
    let json_msg = serde_json::to_string(&update_price_msg).unwrap();
    println!("json_msg: {}", json_msg);
    let _msg_send = MsgSend {
        from_address: sender_addr.clone(),
        to_address: sender_addr.clone(),
        amount: vec![Coin {
            denom: "ukuji".parse().unwrap(),
            amount: 1u128,
        }],
    };
    let execute_msg = MsgExecuteContract {
        sender: sender_addr.clone(),
        contract: contract_addr,
        msg: json_msg.into_bytes(),
        funds: vec![],
    };

    let mut tx_body_builder = tx::BodyBuilder::new();
    tx_body_builder.msg(execute_msg.into_any().unwrap());
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
    println!("chain_id: {}", &chain_id);
    println!("account_number: {}", &account_number);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();
    let tx_signed = sign_doc.sign(&sender_priv_key).unwrap();
    let tx_bytes = tx_signed.to_bytes().unwrap();
    let tx_parsed = Tx::from_bytes(&tx_bytes).unwrap();
    assert_eq!(tx_parsed.body, tx_body);
    assert_eq!(tx_parsed.auth_info, auth_info);
    let rpc_url = var("RPC").unwrap();
    let client = HttpClient::new(rpc_url.as_str()).unwrap();
    let res = tx_signed.broadcast_commit(&client).await.unwrap();
    println!("res: {:#?}", res);
}
