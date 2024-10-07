pub mod block_header;
pub mod constants;
pub mod eras;

use crate::block_header::BlockHeader as VerifiableBlockHeader;

/// Verifies the validity of a block header.
/// This function takes a block number, the block header data (as a `BlockHeader` struct), and the expected block hash for verification.
pub fn verify_block(
    block_number: u64,
    block_header: VerifiableBlockHeader,
    block_hash: &str,
) -> bool {
    match eras::determine_era(block_number) {
        Some(verify_fn) => verify_fn(block_hash.to_string(), block_header),
        None => false, // If the block number is out of the supported range
    }
}
