use reqwest;

const FAUCET_URL: &str = "https://faucet.testnet.aptoslabs.com/mint";
const MAX_AMOUNT: u8 = 10;

struct FaucetRes {
    success: bool,
    code: reqwest::StatusCode,
    msg: String,
}

pub async fn faucet(_account: &Option<String>, _amount: &Option<u8>) {
    println!("===> ✨ Fund faucet to {:?}", _account);
    if _account.is_none() {
        println!("[Error] Missing value account!");
        return;
    }

    let num = _amount.unwrap_or(1);
    let real_amount = match num {
        n if n > MAX_AMOUNT => {
            println!("[Warn] Reset amount to MAX_AMOUNT: {}", MAX_AMOUNT);
            MAX_AMOUNT
        }
        _ => num,
    };

    let mut count: u8 = 1;
    while count <= real_amount {
        println!("===> Round: {}/{} ...", count, real_amount);
        count += 1;
        let res = faucet_one(_account.clone().unwrap(), real_amount).await;
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

async fn faucet_one(_address: String, _amount: u8) -> Result<FaucetRes, reqwest::Error> {
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

    Ok(FaucetRes {
        code,
        success: code == reqwest::StatusCode::OK,
        msg: text,
    })
}
