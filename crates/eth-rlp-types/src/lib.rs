use ethereum_types::H256;
use fixed_hash::rustc_hex::FromHexError;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use thiserror::Error;
use uint::FromHexError as UintFromHexError;

#[derive(Error, Debug)]
pub enum BlockHeaderError {
    #[error("Invalid input length: expected {expected}, got {got}")]
    InvalidInputLength { expected: usize, got: usize },
    #[error("Invalid logs bloom size: expected 256")]
    InvalidLogsBloom,
    #[error("Invalid nonce size")]
    InvalidNonceSize,
    #[error("Rlp decoding error: {0}")]
    RlpDecodingError(#[from] rlp::DecoderError),
    #[error("Hex decoding error: {0}")]
    HexDecodingError(#[from] hex::FromHexError),
    #[error("Rustc hex decoding error: {0}")]
    RustcHexDecodingError(#[from] FromHexError),
    #[error("Uint hex decoding error: {0}")]
    UintHexDecodingError(#[from] UintFromHexError),
}

/// Represents an Ethereum block header with various properties like block hash, gas limits, and more.
///
/// This struct provides a way to store and manipulate data related to an Ethereum block header,
/// including cryptographic information (like the block hash and state root), as well as metadata
/// about the block (such as gas usage and the miner address). It supports fields with optional values,
/// as certain block headers may lack specific information, depending on their context in the chain.
///
/// # Fields
///
/// - `block_hash`: The unique identifier for the block, as a 66-character hexadecimal string (not null).
/// - `number`: The block number, a sequential integer starting from 0 (not null).
/// - `gas_limit`: The maximum amount of gas that can be spent on transactions in this block (not null).
/// - `gas_used`: The actual amount of gas used by transactions within the block (not null).
/// - `nonce`: A 78-character string representing the proof-of-work nonce (not null).
/// - `transaction_root`: The Merkle root of the transactions in the block, if available (optional).
/// - `receipts_root`: The Merkle root of the transaction receipts in the block, if available (optional).
/// - `state_root`: The root of the state trie after transactions in the block are processed (optional).
/// - `base_fee_per_gas`: The base fee per gas for the block in the new fee market mechanism (optional).
/// - `parent_hash`: The hash of the previous block, which this block references (optional).
/// - `ommers_hash`: The hash of the ommers (or uncles), which are alternative blocks that could have been part of the chain (optional).
/// - `miner`: The address of the miner that mined the block (optional).
/// - `logs_bloom`: The bloom filter for logs included in the block, useful for filtering logs efficiently (optional).
/// - `difficulty`: The difficulty level required to mine the block (optional).
/// - `total_difficulty`: The total difficulty of the chain up to this block (optional).
/// - `sha3_uncles`: The SHA3 hash of the uncles (ommers) included in the block (optional).
/// - `timestamp`: The time at which the block was mined, as a string (optional).
/// - `extra_data`: Extra data included in the block, up to 1024 bytes (optional).
/// - `mix_hash`: A hash used to verify the proof of work, ensuring that the mining process was valid (optional).
/// - `withdrawals_root`: The Merkle root of withdrawal data for withdrawals included in the block, if any (optional).
/// - `blob_gas_used`: The amount of blob gas used, specific to blob transactions (optional).
/// - `excess_blob_gas`: The excess blob gas present in the block (optional).
/// - `parent_beacon_block_root`: The root of the parent beacon block, used in Ethereum's proof-of-stake chain (optional).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockHeader {
    pub block_hash: String,               // character(66) NOT NULL
    pub number: i64,                      // bigint NOT NULL
    pub gas_limit: i64,                   // bigint NOT NULL
    pub gas_used: i64,                    // bigint NOT NULL
    pub nonce: String,                    // character varying(78) NOT NULL
    pub transaction_root: Option<String>, // character(66)
    pub receipts_root: Option<String>,    // character(66)
    pub state_root: Option<String>,       // character(66)
    pub base_fee_per_gas: Option<String>, // character varying(78)
    pub parent_hash: Option<String>,
    pub ommers_hash: Option<String>,      // character varying(66)
    pub miner: Option<String>,            // character varying(42)
    pub logs_bloom: Option<String>,       // character varying(1024)
    pub difficulty: Option<String>,       // character varying(78)
    pub totaldifficulty: Option<String>,  // character varying(78)
    pub sha3_uncles: Option<String>,      // character varying(66)
    pub timestamp: Option<String>,        // character varying(100)
    pub extra_data: Option<String>,       // character varying(1024)
    pub mix_hash: Option<String>,         // character varying(66)
    pub withdrawals_root: Option<String>, // character varying(66)
    pub blob_gas_used: Option<String>,    // character varying(78)
    pub excess_blob_gas: Option<String>,  // character varying(78)
    pub parent_beacon_block_root: Option<String>, // character varying(66)
}

/// A trait that defines common behaviors for Ethereum block headers, including RLP encoding and hash computation.
///
/// This trait provides methods to handle standard operations on Ethereum block headers such as encoding
/// and hashing, which are critical for verifying the integrity of a block and its inclusion in the blockchain.
/// It also provides a utility method for converting hexadecimal strings into fixed-size byte arrays, which is
/// useful when handling Ethereum's cryptographic data.
pub trait BlockHeaderTrait {
    /// Encodes the block header using RLP (Recursive Length Prefix) encoding, which is used to serialize Ethereum objects.
    ///
    /// This method should encode the block header into a byte vector using RLP encoding. It is essential for
    /// transmitting the block header in a compact format or for use in cryptographic operations, such as computing
    /// the block hash.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header.
    fn rlp_encode(&self) -> Vec<u8>;

    /// Decodes an RLP-encoded byte vector into a block header struct.
    ///
    /// This function takes an RLP-encoded byte vector and decodes it into a block header.
    ///
    /// # Arguments
    /// - `data`: A byte slice containing the RLP-encoded data.
    ///
    /// # Returns
    /// - A `Result<Self>` which is either the decoded block header or an error if decoding fails.
    fn rlp_decode(data: &[u8]) -> Result<Self, BlockHeaderError>
    where
        Self: Sized;

    /// Computes the Keccak256 hash of the block header.
    ///
    /// This method first encodes the block header using the `rlp_encode` method and then computes the Keccak256
    /// hash of the resulting bytes. This hash is used as the block's unique identifier (`block_hash`) in the Ethereum
    /// blockchain and plays a crucial role in the consensus and security mechanisms.
    ///
    /// # Returns
    ///
    /// An `H256` hash, representing the 32-byte Keccak256 hash of the block header.
    fn compute_hash(&self) -> H256 {
        let rlp_encoded = self.rlp_encode();
        let hash = Keccak256::digest(rlp_encoded);
        H256::from_slice(&hash)
    }

    /// Converts a hexadecimal string to a fixed-size byte array.
    ///
    /// This utility function converts a hexadecimal string (prefixed with "0x") into a fixed-size array of bytes.
    /// It ensures that the length of the byte array matches the expected size, and will panic if the input
    /// string does not have the correct length.
    ///
    /// # Arguments
    ///
    /// - `hex_str`: A string slice containing a hexadecimal string (e.g., "0x1234...").
    ///
    /// # Returns
    ///
    /// A fixed-size array of `N` bytes representing the decoded hexadecimal string.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of the decoded bytes does not match the expected size `N`.
    fn hex_to_fixed_array<const N: usize>(hex_str: &str) -> Result<[u8; N], BlockHeaderError> {
        if hex_str.is_empty() || hex_str == "0x" {
            return Ok([0u8; N]); // Return an empty array if the input is empty
        }

        // Ensure the hex string starts with "0x"
        let content = hex_str.strip_prefix("0x").unwrap_or(hex_str);

        // Pad with a leading '0' if the length is odd
        let padded_content = if content.len() % 2 != 0 {
            format!("0{}", content)
        } else {
            content.to_string()
        };

        // Decode the hex string
        let bytes = hex::decode(&padded_content).map_err(BlockHeaderError::HexDecodingError)?;

        // Check if the decoded length matches the expected size
        if bytes.len() != N {
            return Err(BlockHeaderError::InvalidInputLength {
                expected: N,
                got: bytes.len(),
            });
        }

        // Copy bytes into the fixed-size array
        let mut array = [0u8; N];
        array.copy_from_slice(&bytes);
        Ok(array)
    }
}

struct BlockHeaderImpl;

impl BlockHeaderTrait for BlockHeaderImpl {
    fn rlp_encode(&self) -> Vec<u8> {
        vec![]
    }

    fn rlp_decode(_data: &[u8]) -> Result<Self, BlockHeaderError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hex_to_fixed_array() {
        let hex_str = "0x1234567890abcdef1234567890abcdef";
        let result = BlockHeaderImpl::hex_to_fixed_array::<16>(hex_str);
        let expected: [u8; 16] = [
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab,
            0xcd, 0xef,
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_empty_hex_string() {
        let hex_str = "0x"; // No content after "0x"
        let result = BlockHeaderImpl::hex_to_fixed_array::<4>(hex_str);
        assert_eq!(result.unwrap(), [0u8; 4]);
    }

    #[test]
    fn test_empty_string() {
        let hex_str = ""; // Completely empty string
        let result = BlockHeaderImpl::hex_to_fixed_array::<4>(hex_str);
        assert_eq!(result.unwrap(), [0u8; 4]);
    }

    #[test]
    #[should_panic(expected = "Failed to decode hex string")]
    fn test_invalid_hex_string() {
        let hex_str = "0xGGGG"; // Invalid hex characters
        BlockHeaderImpl::hex_to_fixed_array::<2>(hex_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid input length: expected 8, got 16")]
    fn test_mismatched_array_length() {
        let hex_str = "0x1234567890abcdef1234567890abcdef";
        BlockHeaderImpl::hex_to_fixed_array::<8>(hex_str).unwrap();
    }

    // #[test]
    // #[should_panic(expected = "Invalid input length: expected even length, got odd length")]
    // fn test_invalid_length_hex_to_fixed_array() {
    //     let hex_str = "0x1234567890abcdef1234567890abcde";
    //     BlockHeaderImpl::hex_to_fixed_array::<16>(hex_str);
    // }
}
