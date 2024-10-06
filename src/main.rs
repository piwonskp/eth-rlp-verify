mod block_header;
mod constants;
mod eras;
mod rpc_client;

use clap::Parser;
use dotenv::dotenv;
use eras::determine_era;
use std::env;
use tokio::task::JoinError;
use tracing::info;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(
    name = "Block Header Verifier",
    about = "Verify Ethereum block headers against their hashes."
)]
struct Cli {
    #[arg(long, value_parser = clap::value_parser!(u64))]
    block: u64,
}

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()), // Default to "info" level if RUST_LOG is not set
        )
        .init();

    // Parse the command-line arguments
    let cli = Cli::parse();

    // Get the RPC URL from the environment variable
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

    // Dispatch to the correct era for block verification
    match determine_era(cli.block, rpc_url) {
        Some(era_function) => era_function.await, // Propagate the result from the spawned task
        None => {
            info!("Block number is out of the supported range.");
            Ok(()) // Return an empty success here to match types
        }
    }
}
