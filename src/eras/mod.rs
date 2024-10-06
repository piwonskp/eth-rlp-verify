mod dencun;
mod genesis;
mod london;
mod paris;
mod shapella;

use crate::constants::{
    DENCUN_START, GENESIS_END, LONDON_END, LONDON_START, PARIS_END, PARIS_START, SHAPELLA_END,
    SHAPELLA_START,
};
use std::future::Future;
use std::pin::Pin;
use tokio::task::JoinHandle;

// Re-export each era's structs and verification functions
pub use dencun::verify_dencun;
pub use genesis::verify_genesis;
pub use london::verify_london;
pub use paris::verify_paris;
pub use shapella::verify_shapella;

/// Helper function to spawn the verification future in a `tokio::spawn` call
fn spawn_era_verification<F>(
    block_number: u64,
    rpc_url: String, // Now takes ownership of rpc_url as String
    f: F,
) -> JoinHandle<()>
where
    F: FnOnce(u64, String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static,
{
    tokio::spawn(f(block_number, rpc_url)) // Pass the owned rpc_url
}

/// Determine the era based on the block number and return the appropriate function for verification
pub fn determine_era(block_number: u64, rpc_url: String) -> Option<JoinHandle<()>> {
    // Changed to pass rpc_url as an owned String
    if (LONDON_START..=LONDON_END).contains(&block_number) {
        Some(spawn_era_verification(
            block_number,
            rpc_url.clone(),
            move |block_number, rpc_url| {
                Box::pin(verify_london(block_number, rpc_url))
                    as Pin<Box<dyn Future<Output = ()> + Send>> // Pass &rpc_url as &str
            },
        ))
    } else if (PARIS_START..=PARIS_END).contains(&block_number) {
        Some(spawn_era_verification(
            block_number,
            rpc_url.clone(),
            move |block_number, rpc_url| {
                Box::pin(verify_paris(block_number, rpc_url))
                    as Pin<Box<dyn Future<Output = ()> + Send>> // Pass &rpc_url as &str
            },
        ))
    } else if (SHAPELLA_START..=SHAPELLA_END).contains(&block_number) {
        Some(spawn_era_verification(
            block_number,
            rpc_url.clone(),
            move |block_number, rpc_url| {
                Box::pin(verify_shapella(block_number, rpc_url))
                    as Pin<Box<dyn Future<Output = ()> + Send>> // Pass &rpc_url as &str
            },
        ))
    } else if block_number >= DENCUN_START {
        Some(spawn_era_verification(
            block_number,
            rpc_url.clone(),
            move |block_number, rpc_url| {
                Box::pin(verify_dencun(block_number, rpc_url))
                    as Pin<Box<dyn Future<Output = ()> + Send>> // Pass &rpc_url as &str
            },
        ))
    } else if block_number <= GENESIS_END {
        Some(spawn_era_verification(
            block_number,
            rpc_url.clone(),
            move |block_number, rpc_url| {
                Box::pin(verify_genesis(block_number, rpc_url))
                    as Pin<Box<dyn Future<Output = ()> + Send>> // Pass &rpc_url as &str
            },
        ))
    } else {
        None
    }
}
