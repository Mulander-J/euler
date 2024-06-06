use serde_json::json;
use reqwest;
use super::cmd::SuiNetwork;

const MAX_AMOUNT: u8 = 5;

const FAUCET_DEV_URL: &str = "https://faucet.devnet.sui.io/gas";
const FAUCET_TEST_URL: &str = "https://faucet.testnet.sui.io/gas";

struct ResBody {
    success: bool,
    code: reqwest::StatusCode,
    msg: String,
}

pub async fn faucet(_network: &SuiNetwork, _account: &Option<String>, _amount: &Option<u8>) {
    println!("===> ✨ From {} fund faucet to {:?}",  _network.to_string(), _account);

    if _account.is_none() {
        println!("[Error] Missing value account!");
        return;
    }

    let num = _amount.unwrap_or(1);
    let real_count = match num {
        n if n > MAX_AMOUNT => {
            println!("[Warn] Reset amount to MAX_AMOUNT: {}", MAX_AMOUNT);
            MAX_AMOUNT
        }
        _ => num,
    };

    let faucet_url = match _network {
        SuiNetwork::DEVNET => FAUCET_DEV_URL,
        SuiNetwork::TESTNET => FAUCET_TEST_URL,
    };

    let mut count: u8 = 1;
    while count <= real_count {
        println!("===> Round: {}/{} ...", count, real_count);
        count += 1;
        let res = faucet_one(faucet_url.to_string(), _account.clone().unwrap()).await;
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

async fn faucet_one(_url: String, _address: String) -> Result<ResBody, reqwest::Error> {
    // pack headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    // send request
    let client = reqwest::Client::new();
    let res = client
        .post(_url)
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
        msg: text
    })
}
