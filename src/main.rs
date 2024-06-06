use clap::{Parser, Subcommand};

mod aptos;
mod sui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Aptos utils(e.g. faucet)
    Aptos(aptos::cmd::AptosCli),
    /// Sui utils(e.g. faucet)
    Sui(sui::cmd::SuiCli)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Aptos(sub) => {
            match &sub.command {
                aptos::cmd::AptosCmd::Faucet { account, amount } => {
                    aptos::utils::faucet(account, amount).await
                }
            }
        },
        Commands::Sui(sub) => {
            match &sub.command {
                sui::cmd::SuiCmd::Faucet { network, account,count } => {
                    sui::utils::faucet(network, account, count).await
                }
            }
        }
    }    
}
