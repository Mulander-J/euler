use anyhow::Result;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use reqwest;
use serde::Deserialize;
use std::convert::TryFrom;
use std::sync::Arc;
use std::time::Duration;

const MAX_AMOUNT: u8 = 5;
/// https://api-homepage.kaia.io/faucet/run?address=0x0
const FAUCET_URL: &str = "https://api-baobab.wallet.klaytn.com/faucet/run";
// recaptcha
const PROVIDER_URL: &str = "https://public-en-kairos.node.kaia.io";

#[derive(Deserialize, Debug)]
struct ErrReason {
    code: u32,
    // target: String,
    #[allow(dead_code)]
    result: String,
    #[allow(dead_code)]
    data: String,
}

pub async fn run(_account: &String, _amount: &Option<u8>) {
    println!("===> ✨ Fund faucet to {:?}", _account);
    if !_account.is_empty() {
        println!("[OOPS] Kaia faucet closed.");
        return;
    }

    let num = _amount.unwrap_or(1).min(MAX_AMOUNT);

    let is_bridge = num > 1;
    let mut count: u8 = 1;
    while count <= num {
        println!("===> Round: {}/{} ...", count, num);
        count += 1;

        let res = run_one(_account.clone(), is_bridge).await;
        match res {
            Ok(success) => {
                if !success {
                    break;
                }
            }
            Err(err) => {
                println!("[Error] {:#?}", err);
                break;
            }
        }
    }
}

async fn run_one(collector: String, bridge: bool) -> Result<bool> {
    if !bridge {
        fund(collector).await.map_err(|e| e.into())
    } else {
        // make random wallet(include pubKey + privateKey)
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let address = wallet.address();
        let private_key = au8_to_string(wallet.signer().to_bytes().as_slice().to_vec());
        println!("PubKey: {:?} \n\rPrivateKey: {}", address, private_key);
        let funded = fund(address.to_string()).await?;
        if funded {
            // Set provider
            let provider =
                Provider::<Http>::try_from(PROVIDER_URL)?.interval(Duration::from_millis(10));
            let provider = Arc::new(provider);
            // transfer funds
            let collector_address: Address = collector.parse()?;
            let client = SignerMiddleware::new(provider.clone(), wallet.clone());
            let client = Arc::new(client);
            // build txn
            let balance = provider.get_balance(address, None).await?;
            let tx = TransactionRequest::pay(collector_address, balance);
            let pending_tx = client.send_transaction(tx, None).await?;
            println!("Transfer Hash: {:?}", pending_tx.tx_hash());
            // wait txn
            let receipt = pending_tx.confirmations(1).await?;
            println!("Receipt: {:?}", receipt);
        }
        Ok(funded)
    }
}

async fn fund(_address: String) -> Result<bool> {
    // pack headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    // send request
    let client = reqwest::Client::new();
    let res = client
        .post(FAUCET_URL)
        .headers(headers)
        .query(&[("address", _address)])
        .send()
        .await?;

    // check response
    let code = res.status();
    let json_ret = res.json::<ErrReason>().await?;
    let msg = format!("{:?}", json_ret);

    println!("[Request-{}] {:#?}", code, msg);

    Ok(![993_u32,994_u32,999_u32].contains(&json_ret.code) )
}

fn au8_to_string(signature_code: Vec<u8>) -> String {
    let mut private_key = String::new();
    for a in signature_code.iter() {
        let fstr = format!("{:02x}", a);
        private_key.push_str(&fstr);
    }
    private_key
}
