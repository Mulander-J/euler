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
    /// Aptos utils(e.g. faucet)
    Aptos(aptos::cmd::AptosCli),
    // Sui
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
        }
    }    
}
