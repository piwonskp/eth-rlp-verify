#![deny(unused_crate_dependencies)]

pub mod constants;
pub mod eras;
pub mod test_helpers;
pub mod traits;
use eth_rlp_types::{BlockHeader as VerifiableBlockHeader, BlockHeaderError};

pub fn are_blocks_and_chain_valid(block_headers: &[VerifiableBlockHeader]) -> bool {
    for (i, block) in block_headers.iter().enumerate() {
        let block_hash = block.block_hash.clone();
        let parent_hash = block.parent_hash.clone().unwrap_or_default();
        let block_number = block.number;

        let is_valid = match verify_block(block_number as u64, block.clone(), &block_hash) {
            Ok(valid) => valid,
            Err(_) => false,
        };

        if !is_valid {
            return false;
        }

        if i != 0 {
            let previous_block = &block_headers[i - 1];
            let previous_block_hash = previous_block.block_hash.clone();

            if parent_hash != previous_block_hash {
                return false;
            }
        }
    }

    true
}

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
) -> Result<bool, BlockHeaderError> {
    match eras::determine_era(block_number) {
        Some(verify_fn) => verify_fn(block_hash.to_string(), block_header),
        None => Ok(false),
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
    eras::determine_era_encoder(block_number).and_then(|encoder| encoder(block_header).ok())
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

#[cfg(test)]
mod tests {
    use super::{decode_block_header, encode_block_header};
    use crate::test_helpers::create_test_block_header_london; // Adjust import as needed

    #[test]
    #[ignore]
    fn test_block_header_encoding_decoding_debug() {
        // Create the test block header for London era
        let original_header = create_test_block_header_london();
        let block_number = original_header.number as u64;

        println!("Original Header: {:#?}", original_header);

        // Step 1: Encode the block header
        let encoded =
            encode_block_header(block_number, original_header.clone()).expect("Encoding failed");

        println!("Encoded Bytes: {:?}", encoded);

        // Debug: Reprint encoded bytes as hex
        let encoded_hex: String = encoded.iter().map(|b| format!("{:02x}", b)).collect();
        println!("Encoded Hex: {}", encoded_hex);

        // Step 2: Decode the block header
        let decoded_header = decode_block_header(block_number, &encoded).expect("Decoding failed");

        println!("Decoded Header: {:#?}", decoded_header);

        // Step 3: Compare individual fields for discrepancies
        if original_header.parent_hash != decoded_header.parent_hash {
            println!(
                "Mismatch in parent_hash:\nOriginal: {}\nDecoded: {}",
                original_header.parent_hash.clone().unwrap_or_default(),
                decoded_header.parent_hash.clone().unwrap_or_default()
            );
        }
        assert_eq!(
            original_header.parent_hash, decoded_header.parent_hash,
            "Mismatch in parent_hash"
        );

        assert_eq!(
            original_header.nonce, decoded_header.nonce,
            "Mismatch in nonce"
        );

        assert_eq!(
            original_header.base_fee_per_gas, decoded_header.base_fee_per_gas,
            "Mismatch in base_fee_per_gas"
        );

        assert_eq!(
            original_header, decoded_header,
            "The original and decoded headers do not match"
        );
    }
}
