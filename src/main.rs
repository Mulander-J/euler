use clap::{Parser, Subcommand};

mod aptos;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get Aptos faucet as maximum
    Aptos {
        /// Address to fund
        account: Option<String>,
        ///  Number of Apt to fund the account from the faucet [1~10, default: 1]
        amount: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Aptos { account, amount } => {
            aptos::utils::faucet(account, amount);
        }
    }
}
