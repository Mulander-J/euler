use super::_type::ResBody;
use reqwest;
use serde_json::json;

// const FAUCET_URL: &str = "https://faucet.testnet.aptoslabs.com/mint";
const FAUCET_URL: &str = "https://faucet.testnet.aptoslabs.com/fund";
const MAX_AMOUNT: u8 = 10;

pub async fn run(_account: &String, _token: &String, _count: &Option<u8>) {
    println!("===> ✨  Get airdrop to 🤖 {:?}", _account);
    let num = _count.unwrap_or(MAX_AMOUNT).min(MAX_AMOUNT);
    let mut count: u8 = 1;
    while count <= num {
        println!("===> Round: {}/{} ...", count, num);
        count += 1;
        match run_one(_account.clone(), _token).await {
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

async fn run_one(_address: String, _token: &String) -> Result<ResBody, reqwest::Error> {
    let headers = reqwest::header::HeaderMap::from_iter(vec![
        // (reqwest::header::HeaderName::from_static("content-length"), "0".parse().unwrap()),
        (
            reqwest::header::HeaderName::from_static("content-type"),
            "application/json".parse().unwrap(),
        ),
        (
            reqwest::header::HeaderName::from_static("x-is-jwt"),
            "true".parse().unwrap(),
        ),
        (
            reqwest::header::HeaderName::from_static("authorization"),
            _token.parse().unwrap(),
        ),
    ]);

    let client = reqwest::Client::new();
    let res = client
        .post(FAUCET_URL)
        .headers(headers)
        .json(&json!({
            "address": _address
        }))
        .send()
        .await?;

    let code = res.status();
    let text = res.text().await?;

    Ok(ResBody {
        code,
        success: code == reqwest::StatusCode::OK,
        msg: text,
    })
}
