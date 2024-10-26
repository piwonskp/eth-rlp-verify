#![deny(unused_crate_dependencies)]

pub mod block_header;
pub mod constants;
pub mod eras;
pub mod test_helpers;
pub mod traits;
use crate::block_header::BlockHeader as VerifiableBlockHeader;

/// Verifies the validity of an Ethereum block header based on the block number and expected hash.
///
/// This function determines the appropriate Ethereum era based on the block number, retrieves the corresponding
/// verification function, and verifies the block header by comparing its computed hash with the expected block hash.
/// The verification process ensures that the block is authentic and belongs to the correct place in the blockchain.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the block number of the block being verified.
/// - `block_header`: A `VerifiableBlockHeader` struct containing the block header data that needs to be verified.
/// - `block_hash`: A string slice representing the expected hash of the block.
///
/// # Returns
///
/// A `bool` indicating whether the block header is valid.
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

/// Encodes an Ethereum block header into RLP format.
///
/// This function determines the correct era based on the block number and encodes
/// the block header accordingly.
///
/// # Returns
///
/// An `Option<Vec<u8>>` containing the RLP-encoded block header data if successful.
pub fn encode_block_header(
    block_number: u64,
    block_header: VerifiableBlockHeader,
) -> Option<Vec<u8>> {
    eras::determine_era_encoder(block_number).map(|encoder| encoder(block_header))
}

/// Decodes an RLP-encoded block header based on the block number.
///
/// This function determines the correct era for the given block number and decodes the RLP-encoded
/// data into a `VerifiableBlockHeader`. If the block number is not within a recognized era or decoding
/// fails, it returns `None`.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the block number of the block being decoded.
/// - `encoded`: A byte slice containing the RLP-encoded block header data.
///
/// # Returns
///
/// An `Option<VerifiableBlockHeader>` containing the decoded block header if successful.
pub fn decode_block_header(block_number: u64, encoded: &[u8]) -> Option<VerifiableBlockHeader> {
    eras::determine_era_decoder(block_number).and_then(|decoder| decoder(encoded).ok())
}
