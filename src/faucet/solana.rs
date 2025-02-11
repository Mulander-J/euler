use super::_type::ResBody;
use clap::ValueEnum;
use reqwest;
use serde_json::json;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SolCluster {
    DEVNET,
    TESTNET,
}

impl fmt::Display for SolCluster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolCluster::DEVNET => write!(f, "devnet"),
            SolCluster::TESTNET => write!(f, "testnet"),
        }
    }
}

pub async fn run(_account: &String, _network: &Option<SolCluster>) {
    let network = _network.unwrap_or(SolCluster::TESTNET);
    let url = format!("https://api.{}.solana.com", network.to_string());

    println!(
        "===> ✨  Get airdrop from 🌏 {} to 🤖 {:?}",
        network.to_string(),
        _account
    );

    match run_one(_account.clone(), url).await {
        Ok(frs) => {
            if frs.success {
                println!(
                    "[✅Request-OK] {:#?}\r\nPlease check balance manually(maybe failed).",
                    frs.msg
                );
            } else {
                println!("[❗Request-{}] {:#?}", frs.code, frs.msg);
            }
        }
        Err(err) => {
            println!("[‼️Error] {:#?}", err);
        }
    }
}

async fn run_one(_address: String, _url: String) -> Result<ResBody, reqwest::Error> {
    // send request
    let client = reqwest::Client::new();
    let res = client
        .post(_url)
        .headers(reqwest::header::HeaderMap::from_iter(vec![(
            reqwest::header::HeaderName::from_static("content-type"),
            "application/json".parse().unwrap(),
        )]))
        .json(&json!({
            "jsonrpc":"2.0",
            "id":"1",
            "method": "requestAirdrop",
            "params":[_address,1000000000]
        }))
        .send()
        .await?;

    // check response
    let code = res.status();
    let text = res.text().await?;

    Ok(ResBody {
        code,
        success: code == reqwest::StatusCode::OK,
        msg: text,
    })
}
