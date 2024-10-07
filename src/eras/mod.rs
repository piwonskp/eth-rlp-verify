mod dencun;
mod genesis;
mod london;
mod paris;
mod shapella;

use crate::block_header::BlockHeader as VerifiableBlockHeader;
use crate::constants::{
    DENCUN_START, GENESIS_END, LONDON_END, LONDON_START, PARIS_END, PARIS_START, SHAPELLA_END,
    SHAPELLA_START,
};

// Re-export each era's verification function
pub use dencun::verify_hash_dencun;
pub use genesis::verify_hash_genesis;
pub use london::verify_hash_london;
pub use paris::verify_hash_paris;
pub use shapella::verify_hash_shapella;

/// Determine the era based on the block number and return the appropriate function for verification.
pub fn determine_era(block_number: u64) -> Option<fn(String, VerifiableBlockHeader) -> bool> {
    if (LONDON_START..=LONDON_END).contains(&block_number) {
        Some(verify_hash_london)
    } else if (PARIS_START..=PARIS_END).contains(&block_number) {
        Some(verify_hash_paris)
    } else if (SHAPELLA_START..=SHAPELLA_END).contains(&block_number) {
        Some(verify_hash_shapella)
    } else if block_number >= DENCUN_START {
        Some(verify_hash_dencun)
    } else if block_number <= GENESIS_END {
        Some(verify_hash_genesis)
    } else {
        None
    }
}
