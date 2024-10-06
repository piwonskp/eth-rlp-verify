use ethereum_types::H256;
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct BlockHeader {
    pub block_hash: String,                       // character(66) NOT NULL
    pub number: i64,                              // bigint NOT NULL
    pub gas_limit: i64,                           // bigint NOT NULL
    pub gas_used: i64,                            // bigint NOT NULL
    pub nonce: String,                            // character varying(78) NOT NULL
    pub transaction_root: Option<String>,         // character(66)
    pub receipts_root: Option<String>,            // character(66)
    pub state_root: Option<String>,               // character(66)
    pub base_fee_per_gas: Option<String>,         // character varying(78)
    pub parent_hash: Option<String>,              // character varying(66)
    pub miner: Option<String>,                    // character varying(42)
    pub logs_bloom: Option<String>,               // character varying(1024)
    pub difficulty: Option<String>,               // character varying(78)
    pub total_difficulty: Option<String>,         // character varying(78)
    pub sha3_uncles: Option<String>,              // character varying(66)
    pub timestamp: Option<String>,                // character varying(100)
    pub extra_data: Option<String>,               // character varying(1024)
    pub mix_hash: Option<String>,                 // character varying(66)
    pub withdrawals_root: Option<String>,         // character varying(66)
    pub blob_gas_used: Option<String>,            // character varying(78)
    pub excess_blob_gas: Option<String>,          // character varying(78)
    pub parent_beacon_block_root: Option<String>, // character varying(66)
}

/// A trait defining common behavior for block headers.
pub trait BlockHeaderTrait {
    /// Encodes the block header using RLP encoding.
    fn rlp_encode(&self) -> Vec<u8>;

    /// Computes the hash of the block header.
    fn compute_hash(&self) -> H256 {
        let rlp_encoded = self.rlp_encode();
        let hash = Keccak256::digest(rlp_encoded);
        H256::from_slice(&hash)
    }

    /// Converts a hexadecimal string to a fixed-size array.
    ///
    /// # Arguments
    ///
    /// * `hex_str` - The hexadecimal string to convert.
    fn hex_to_fixed_array<const N: usize>(hex_str: &str) -> [u8; N] {
        let bytes = hex::decode(&hex_str[2..]).unwrap();
        let mut array = [0u8; N];
        let len = bytes.len();
        if len != N {
            panic!("Invalid input length: expected {}, got {}", N, len);
        }
        array.copy_from_slice(&bytes);
        array
    }
}
