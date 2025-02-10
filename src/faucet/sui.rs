use super::_type::ResBody;
use clap::ValueEnum;
use reqwest;
use serde_json::json;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SuiNetwork {
    DEVNET,
    TESTNET,
}

impl fmt::Display for SuiNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuiNetwork::DEVNET => write!(f, "♾️ Devnet"),
            SuiNetwork::TESTNET => write!(f, "🤖 Testnet"),
        }
    }
}

const MAX_AMOUNT: u8 = 5;

const FAUCET_DEV_URL: &str = "https://faucet.devnet.sui.io/gas";
const FAUCET_TEST_URL: &str = "https://faucet.testnet.sui.io/gas";

pub async fn run(_account: &String, _amount: &Option<u8>, _network: &Option<SuiNetwork>) {
    let network = _network.unwrap_or(SuiNetwork::TESTNET);

    println!(
        "===> ✨ From {} fund faucet to {:?}",
        network.to_string(),
        _account
    );

    let num = _amount.unwrap_or(1).min(MAX_AMOUNT);

    let mut count: u8 = 1;
    while count <= num {
        println!("===> Round: {}/{} ...", count, num);
        count += 1;
        let res = run_one(_account.clone(), network).await;
        match res {
            Ok(frs) => {
                if frs.success {
                    println!("[Request-OK] {:#?}", frs.msg);
                } else {
                    println!("[Request-{}] {:#?}", frs.code, frs.msg);
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

async fn run_one(_address: String, _network: SuiNetwork) -> Result<ResBody, reqwest::Error> {
    let faucet_url = match _network {
        SuiNetwork::DEVNET => FAUCET_DEV_URL,
        SuiNetwork::TESTNET => FAUCET_TEST_URL,
    };
    // pack headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    // send request
    let client = reqwest::Client::new();
    let res = client
        .post(faucet_url)
        .headers(headers)
        .json(&json!({"FixedAmountRequest": {"recipient": _address }}))
        .send()
        .await?;

    // check response
    let code = res.status();
    let text = res.text().await?;

    Ok(ResBody {
        code,
        success: code == reqwest::StatusCode::CREATED,
        msg: text,
    })
}
