use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct AptosCli {
    #[command(subcommand)]
    pub command: AptosCmd,
}

#[derive(Subcommand)]
pub enum AptosCmd {
    /// Loop the call to faucet on the aptos-testnet until the maximum as 10
    Faucet {
        /// Address to fund
        account: Option<String>,
        ///  Number of Apt to fund the account from the faucet(1~10, default: 1)
        amount: Option<u8>,
    },
}
