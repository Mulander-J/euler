use reqwest;
use super::_type::ResBody;

const FAUCET_URL: &str = "https://faucet.testnet.aptoslabs.com/mint";
const MAX_AMOUNT: u8 = 10;

pub async fn run(_account: &String, _amount: &Option<u8>) {
    println!("===> ✨ Fund faucet to {:?}", _account);

    let num = _amount.unwrap_or(MAX_AMOUNT).min(MAX_AMOUNT);

    let mut count: u8 = 1;
    while count <= num {
        println!("===> Round: {}/{} ...", count, num);
        count += 1;
        let res = run_one(_account.clone(), num).await;
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

async fn run_one(_address: String, _amount: u8) -> Result<ResBody, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Length",
        reqwest::header::HeaderValue::from_static("0"),
    );

    let client = reqwest::Client::new();
    let res = client
        .post(FAUCET_URL)
        .headers(headers)
        .query(&[
            ("address", _address),
            ("amount", (_amount as u128 * 1000000000).to_string())
        ])
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
