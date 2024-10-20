use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use eyre::Result;
use rlp::{Rlp, RlpStream};
use std::str::FromStr;
use tracing::debug;
/// Represents an Ethereum block header for the London upgrade.
///
/// The London upgrade introduced EIP-1559, a major change to the gas fee mechanism, and
/// this struct contains the relevant fields for verifying and managing Ethereum block headers
/// during this era. Each field represents critical data for ensuring the correctness and
/// consistency of blocks.
///
/// # Fields
///
/// - `parent_hash`: The hash of the previous block, linking this block to the blockchain.
/// - `ommers_hash`: The hash of the ommer (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner/validator who produced the block.
/// - `state_root`: The root of the state trie after processing all transactions.
/// - `transactions_root`: The root hash of the Merkle tree containing all transactions in the block.
/// - `receipts_root`: The root hash of the Merkle tree containing transaction receipts in the block.
/// - `logs_bloom`: A 256-byte bloom filter used for efficiently filtering and searching logs.
/// - `difficulty`: The difficulty required to mine this block, used in proof-of-work (PoW).
/// - `number`: The block number, which identifies its position in the blockchain.
/// - `gas_limit`: The maximum amount of gas allowed to be used by transactions in the block.
/// - `gas_used`: The actual gas consumed by the transactions within this block.
/// - `timestamp`: The time when the block was mined.
/// - `extra_data`: Arbitrary data provided by the miner, with a maximum size of 32 bytes.
/// - `mix_hash`: A PoW-related hash used to verify the mining process.
/// - `nonce`: A 64-bit PoW nonce used in the block's PoW verification.
/// - `base_fee_per_gas`: The minimum gas price per unit introduced by EIP-1559 during the London upgrade.
#[derive(Debug, PartialEq)]
pub struct BlockHeaderLondon {
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

impl BlockHeaderLondon {
    /// Converts a `VerifiableBlockHeader` (typically fetched from the database) into a `BlockHeaderLondon`.
    ///
    /// This function maps a database block header into a `BlockHeaderLondon` struct by parsing
    /// various string fields such as `logs_bloom`, `nonce`, and cryptographic fields like `parent_hash`.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` containing the raw data of the block header.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderLondon` instance with the fields populated and parsed.
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderLondon {
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

    /// Converts a `BlockHeaderLondon` into a `VerifiableBlockHeader`.
    ///
    /// This method ensures that the London-specific block header structure is converted into
    /// the generic `VerifiableBlockHeader` structure used throughout the application.
    pub fn into_verifiable(self) -> VerifiableBlockHeader {
        VerifiableBlockHeader {
            block_hash: "".to_string(), // Placeholder, computed if necessary
            parent_hash: Some(self.parent_hash.to_string()),
            ommers_hash: Some(self.ommers_hash.to_string()),
            miner: Some(self.beneficiary.to_string()),
            state_root: Some(self.state_root.to_string()),
            transaction_root: Some(self.transactions_root.to_string()),
            receipts_root: Some(self.receipts_root.to_string()),
            logs_bloom: Some(hex::encode(self.logs_bloom)),
            difficulty: Some(self.difficulty.to_string()),
            totaldifficulty: None, // Not applicable for London
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

/// Implements the `BlockHeaderTrait` for `BlockHeaderLondon`.
///
/// This implementation enables encoding of the London block header using RLP (Recursive Length Prefix),
/// a compact serialization format used in Ethereum. This method allows for serialization, verification,
/// and the comparison of block headers.
impl BlockHeaderTrait for BlockHeaderLondon {
    /// RLP encodes the London block header, returning a vector of bytes.
    ///
    /// This function serializes all 16 fields of the London block header into RLP format, which is used
    /// for compact storage and transmission. The RLP encoding is critical for Ethereum block verification
    /// and consensus.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(16); // 16 fields in London block header
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
        Ok(BlockHeaderLondon {
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

/// Verifies the hash of a London block by comparing the computed hash with the provided hash.
///
/// This function verifies the integrity of a block by first converting a `VerifiableBlockHeader`
/// to a `BlockHeaderLondon`, then RLP encoding the block header and computing its hash using the
/// Keccak256 hashing algorithm. It compares the computed hash with the given `block_hash`.
///
/// # Arguments
///
/// - `block_hash`: A string containing the expected hash of the block.
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw block header data.
///
/// # Returns
///
/// A boolean value indicating whether the computed hash matches the provided hash.
pub fn verify_hash_london(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderLondon::from_db_header(db_header);

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

    fn mock_block_header_london() -> BlockHeaderLondon {
        BlockHeaderLondon {
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
    fn test_encode_decode_london() {
        let header = mock_block_header_london();
        let encoded = header.rlp_encode();
        let decoded = BlockHeaderLondon::rlp_decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }
}
