use std::fmt;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct SuiCli {
    #[command(subcommand)]
    pub command: SuiCmd,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SuiNetwork {
    DEVNET,
    TESTNET,
}

impl fmt::Display for SuiNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        SuiNetwork::DEVNET => write!(f, "♾️ Devnet"),
        SuiNetwork::TESTNET => write!(f, "🤖 Testnet")
       }
    }
}

#[derive(Subcommand)]
pub enum SuiCmd {
    /// Loop the call fund on the sui faucet until the maximum as 5
    Faucet {
        // Which Network
        #[arg(value_enum)]
        network: SuiNetwork,
        /// Address to fund
        account: Option<String>,
        ///  Times of Sui to fund the account from the faucet(1~5, default: 1)
        count: Option<u8>,
    },
}

