use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use eyre::Result;
use rlp::{Rlp, RlpStream};
use std::str::FromStr;
use tracing::debug;
/// Represents an Ethereum block header for the Paris upgrade.
///
/// The Paris upgrade marks Ethereum's transition from proof-of-work (PoW) to proof-of-stake (PoS),
/// and this struct stores key properties of a block header during that period. It contains fields
/// that describe various cryptographic roots, gas limits, and mining information required to verify
/// and authenticate the block.
///
/// # Fields
///
/// - `parent_hash`: The hash of the parent block, which links this block to the previous one.
/// - `ommers_hash`: The hash of ommer (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner or validator who produced this block.
/// - `state_root`: The root hash of the state trie after this block is processed.
/// - `transactions_root`: The root hash of the Merkle tree of transactions in this block.
/// - `receipts_root`: The root hash of the Merkle tree of transaction receipts for this block.
/// - `logs_bloom`: A bloom filter used for fast log searching, filtering relevant logs efficiently.
/// - `difficulty`: The difficulty value that was required to mine the block in proof-of-work.
/// - `number`: The block number, indicating its position in the blockchain.
/// - `gas_limit`: The maximum gas allowed to be consumed by transactions in this block.
/// - `gas_used`: The total amount of gas used by the transactions in the block.
/// - `timestamp`: The timestamp indicating when the block was mined.
/// - `extra_data`: Extra data associated with the block, typically set by the miner, up to 32 bytes.
/// - `mix_hash`: A hash used to verify the proof-of-work (PoW) mining result.
/// - `nonce`: The 64-bit nonce used to verify the PoW and mine the block.
/// - `base_fee_per_gas`: The minimum gas fee for transactions in this block, as defined in EIP-1559.
#[derive(Debug, PartialEq)]
pub struct BlockHeaderParis {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: [u8; 256],
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Vec<u8>,
    pub mix_hash: H256,
    pub nonce: [u8; 8],
    pub base_fee_per_gas: U256,
}

impl BlockHeaderParis {
    /// Converts a `VerifiableBlockHeader` fetched from the database into a `BlockHeaderParis`.
    ///
    /// This function transforms a `VerifiableBlockHeader` into a `BlockHeaderParis` structure,
    /// parsing necessary fields such as the state root, transactions root, and miner's address.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw data.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderParis` instance with parsed and populated fields.
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderParis {
            parent_hash: H256::from_str(&db_header.parent_hash.unwrap_or_default()).unwrap(),
            ommers_hash: H256::from_str(&db_header.sha3_uncles.unwrap_or_default()).unwrap(),
            beneficiary: H160::from_str(&db_header.miner.unwrap_or_default()).unwrap(),
            state_root: H256::from_str(&db_header.state_root.unwrap_or_default()).unwrap(),
            transactions_root: H256::from_str(&db_header.transaction_root.unwrap_or_default())
                .unwrap(),
            receipts_root: H256::from_str(&db_header.receipts_root.unwrap_or_default()).unwrap(),
            logs_bloom,
            difficulty: U256::from_str(&db_header.difficulty.unwrap_or("0x0".to_string())).unwrap(),
            number: U256::from(db_header.number as u64),
            gas_limit: U256::from(db_header.gas_limit as u64),
            gas_used: U256::from(db_header.gas_used as u64),
            timestamp: U256::from_str(&db_header.timestamp.unwrap_or_default()).unwrap(),
            extra_data: hex::decode(&db_header.extra_data.unwrap_or_default()[2..])
                .unwrap_or_default(),
            mix_hash: H256::from_str(&db_header.mix_hash.unwrap_or_default()).unwrap(),
            nonce,
            base_fee_per_gas: U256::from_str(&db_header.base_fee_per_gas.unwrap_or_default())
                .unwrap(),
        }
    }

    /// Converts a `BlockHeaderParis` into a common `VerifiableBlockHeader`.
    ///
    /// This method ensures that the Paris-specific block header structure is converted into the
    /// generic `VerifiableBlockHeader` structure used throughout the application.
    pub fn into_verifiable(self) -> VerifiableBlockHeader {
        VerifiableBlockHeader {
            block_hash: "".to_string(), // Placeholder; compute if necessary.
            parent_hash: Some(self.parent_hash.to_string()),
            ommers_hash: Some(self.ommers_hash.to_string()),
            miner: Some(self.beneficiary.to_string()),
            state_root: Some(self.state_root.to_string()),
            transaction_root: Some(self.transactions_root.to_string()),
            receipts_root: Some(self.receipts_root.to_string()),
            logs_bloom: Some(hex::encode(self.logs_bloom)),
            difficulty: Some(self.difficulty.to_string()),
            totaldifficulty: None, // Not applicable for Paris.
            number: self.number.as_u64() as i64,
            gas_limit: self.gas_limit.as_u64() as i64,
            gas_used: self.gas_used.as_u64() as i64,
            timestamp: Some(self.timestamp.to_string()),
            extra_data: Some(hex::encode(self.extra_data)),
            mix_hash: Some(self.mix_hash.to_string()),
            nonce: hex::encode(self.nonce),
            base_fee_per_gas: Some(self.base_fee_per_gas.to_string()),
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
            parent_beacon_block_root: None,
            sha3_uncles: None,
        }
    }
}

/// Implements the `BlockHeaderTrait` for `BlockHeaderParis`.
///
/// This trait implementation enables RLP encoding for the Paris block header, which is essential
/// for compact and efficient serialization. It ensures that the Paris block header can be serialized
/// and verified using Ethereum's standard methods.
impl BlockHeaderTrait for BlockHeaderParis {
    /// RLP encodes the Paris block header, producing a vector of bytes.
    ///
    /// This function encodes all 16 fields of the Paris block header in compliance with
    /// Ethereum's RLP encoding scheme, which is used for serialization and block verification.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(16); // 16 fields in Paris block header
        stream.append(&self.parent_hash);
        stream.append(&self.ommers_hash);
        stream.append(&self.beneficiary);
        stream.append(&self.state_root);
        stream.append(&self.transactions_root);
        stream.append(&self.receipts_root);
        stream.append(&self.logs_bloom.to_vec());
        stream.append(&self.difficulty);
        stream.append(&self.number);
        stream.append(&self.gas_limit);
        stream.append(&self.gas_used);
        stream.append(&self.timestamp);
        stream.append(&self.extra_data);
        stream.append(&self.mix_hash);
        stream.append(&self.nonce.as_slice());
        stream.append(&self.base_fee_per_gas);
        stream.out().to_vec()
    }

    fn rlp_decode(data: &[u8]) -> Result<Self> {
        let rlp = Rlp::new(data);
        Ok(BlockHeaderParis {
            parent_hash: rlp.val_at(0)?,
            ommers_hash: rlp.val_at(1)?,
            beneficiary: rlp.val_at(2)?,
            state_root: rlp.val_at(3)?,
            transactions_root: rlp.val_at(4)?,
            receipts_root: rlp.val_at(5)?,
            logs_bloom: rlp
                .val_at::<Vec<u8>>(6)?
                .try_into()
                .map_err(|_| eyre::eyre!("Invalid logs_bloom size"))?,
            difficulty: rlp.val_at(7)?,
            number: rlp.val_at(8)?,
            gas_limit: rlp.val_at(9)?,
            gas_used: rlp.val_at(10)?,
            timestamp: rlp.val_at(11)?,
            extra_data: rlp.val_at(12)?,
            mix_hash: rlp.val_at(13)?,
            nonce: rlp
                .val_at::<Vec<u8>>(14)?
                .try_into()
                .map_err(|_| eyre::eyre!("Invalid nonce size"))?,
            base_fee_per_gas: rlp.val_at(15)?,
        })
    }
}

/// Verifies the integrity of a Paris block hash by comparing it with the computed hash.
///
/// This function uses the provided block hash and the database block header to create a `BlockHeaderParis`,
/// then computes the block hash by RLP encoding the header and applying the Keccak256 hashing algorithm.
/// The computed hash is compared to the provided `block_hash` to check if the block is valid.
///
/// # Arguments
///
/// - `block_hash`: The expected hash of the block (as a hexadecimal string).
/// - `db_header`: A `VerifiableBlockHeader` containing block header information from the database.
///
/// # Returns
///
/// A boolean indicating whether the computed block hash matches the provided hash.
pub fn verify_hash_paris(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderParis::from_db_header(db_header);

    // Log the RLP encoded data for debugging purposes
    let rlp_encoded = header.rlp_encode();
    debug!("RLP Encoded: {:?}", rlp_encoded);

    // Compute the block hash
    let computed_block_hash = header.compute_hash();
    debug!("Computed Block Hash: {:?}", computed_block_hash);

    // Check if the computed hash matches the given block hash
    let is_valid = computed_block_hash == H256::from_str(&block_hash).unwrap();
    debug!("Is the block hash valid? {}", is_valid);
    is_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_block_header_paris() -> BlockHeaderParis {
        BlockHeaderParis {
            parent_hash: H256::zero(),
            ommers_hash: H256::zero(),
            beneficiary: H160::zero(),
            state_root: H256::zero(),
            transactions_root: H256::zero(),
            receipts_root: H256::zero(),
            logs_bloom: [0; 256],
            difficulty: U256::zero(),
            number: U256::zero(),
            gas_limit: U256::zero(),
            gas_used: U256::zero(),
            timestamp: U256::zero(),
            extra_data: vec![],
            mix_hash: H256::zero(),
            nonce: [0; 8],
            base_fee_per_gas: U256::zero(),
        }
    }

    #[test]
    fn test_encode_decode_paris() {
        let header = mock_block_header_paris();
        let encoded = header.rlp_encode();
        let decoded = BlockHeaderParis::rlp_decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }
}
