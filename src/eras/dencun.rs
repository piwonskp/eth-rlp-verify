use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use eyre::Result;
use rlp::{Rlp, RlpStream};
use std::str::FromStr;
use tracing::debug;
/// Represents the block header for the Dencun upgrade in Ethereum.
///
/// The Dencun upgrade introduces several new fields to the Ethereum block header, such as `parent_beacon_block_root`,
/// `blob_gas_used`, and `excess_blob_gas`. This struct holds the critical fields for verifying and validating
/// blocks from the Dencun era, which marks significant changes in the Ethereum consensus mechanism.
///
/// # Fields
///
/// - `parent_hash`: The hash of the parent block, linking this block to the blockchain.
/// - `ommers_hash`: The hash of the ommers (uncles) included in this block.
/// - `beneficiary`: The Ethereum address of the miner who produced the block.
/// - `state_root`: The root of the state trie after the block is processed.
/// - `transactions_root`: The Merkle root of the transactions included in the block.
/// - `receipts_root`: The root of the transaction receipts for transactions in the block.
/// - `logs_bloom`: A bloom filter used for efficient searching and filtering of logs, 256 bytes in size.
/// - `difficulty`: The difficulty level for mining this block, used in proof-of-work (PoW) algorithms.
/// - `number`: The block number, which indicates its position in the blockchain.
/// - `gas_limit`: The maximum gas allowed to be consumed by transactions in this block.
/// - `gas_used`: The actual gas used by the transactions in this block.
/// - `timestamp`: The time when the block was mined.
/// - `extra_data`: Arbitrary data added by the miner, up to 32 bytes in length.
/// - `mix_hash`: A hash used in the proof-of-work algorithm for verifying the mining process.
/// - `nonce`: A 64-bit nonce used to validate the mining process in the proof-of-work consensus mechanism.
/// - `base_fee_per_gas`: The base gas fee per unit for transactions in this block, part of EIP-1559.
/// - `withdrawals_root`: The Merkle root of the withdrawals processed in the block (introduced in Shapella).
/// - `parent_beacon_block_root`: The root of the parent beacon block, newly introduced in the Dencun upgrade.
/// - `blob_gas_used`: The gas used for blob-related transactions (new in Dencun).
/// - `excess_blob_gas`: The excess blob gas in the block, used to manage blob-related transaction fees (new in Dencun).
#[derive(Debug, PartialEq)]
pub struct BlockHeaderDencun {
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
    pub withdrawals_root: H256,
    pub parent_beacon_block_root: H256, // New in Dencun
    pub blob_gas_used: U256,            // New in Dencun
    pub excess_blob_gas: U256,          // New in Dencun
}

impl BlockHeaderDencun {
    /// Converts a `VerifiableBlockHeader` from the database into a `BlockHeaderDencun`.
    ///
    /// This method takes a block header from the database, parses its fields, and converts it
    /// into a `BlockHeaderDencun` struct. It handles conversion of fields such as `logs_bloom`,
    /// `nonce`, and various cryptographic root fields, ensuring that the header is ready for verification.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw string data
    ///   for the block header fields.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderDencun` instance populated with the parsed and validated block header data.
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderDencun {
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
            withdrawals_root: H256::from_str(&db_header.withdrawals_root.unwrap_or_default())
                .unwrap(),
            parent_beacon_block_root: H256::from_str(
                &db_header.parent_beacon_block_root.unwrap_or_default(),
            )
            .unwrap_or_default(),
            blob_gas_used: U256::from_str(&db_header.blob_gas_used.unwrap_or_default())
                .unwrap_or_default(),
            excess_blob_gas: U256::from_str(&db_header.excess_blob_gas.unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    /// Converts a `BlockHeaderDencun` into a common `BlockHeader`.
    ///
    /// This method ensures that the Dencun-specific block header structure is converted into the
    /// generic `BlockHeader` structure used throughout the application.
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
            totaldifficulty: None, // Not available in Dencun.
            number: self.number.as_u64() as i64,
            gas_limit: self.gas_limit.as_u64() as i64,
            gas_used: self.gas_used.as_u64() as i64,
            timestamp: Some(self.timestamp.to_string()),
            extra_data: Some(hex::encode(self.extra_data)),
            mix_hash: Some(self.mix_hash.to_string()),
            nonce: hex::encode(self.nonce),
            base_fee_per_gas: Some(self.base_fee_per_gas.to_string()),
            withdrawals_root: Some(self.withdrawals_root.to_string()),
            parent_beacon_block_root: Some(self.parent_beacon_block_root.to_string()),
            blob_gas_used: Some(self.blob_gas_used.to_string()),
            excess_blob_gas: Some(self.excess_blob_gas.to_string()),
            sha3_uncles: None, // Not applicable for Dencun.
        }
    }
}

/// Implements the `BlockHeaderTrait` for `BlockHeaderDencun`.
///
/// This implementation provides RLP encoding for the Dencun block header, which is necessary for
/// compact serialization and for verifying blocks on the Ethereum network. This trait enables
/// serialization and hash verification, critical for validating blocks.
impl BlockHeaderTrait for BlockHeaderDencun {
    /// RLP encodes the Dencun block header, returning a vector of bytes.
    ///
    /// This method serializes all 20 fields of the Dencun block header using Ethereum's
    /// RLP (Recursive Length Prefix) encoding. This encoding is essential for compact
    /// storage and transmission, and is used for verifying block consistency and integrity.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(20); // 20 fields in Dencun block header
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
        stream.append(&self.withdrawals_root);
        stream.append(&self.blob_gas_used);
        stream.append(&self.excess_blob_gas);
        stream.append(&self.parent_beacon_block_root);
        stream.out().to_vec()
    }

    /// Decodes an RLP-encoded byte slice into a `BlockHeaderDencun`.
    ///
    /// # Arguments
    /// - `data`: A byte slice containing the RLP-encoded data.
    ///
    /// # Returns
    /// - A `Result<Self>` containing the decoded block header or an error if decoding fails.
    fn rlp_decode(data: &[u8]) -> Result<Self> {
        let rlp = Rlp::new(data);
        Ok(BlockHeaderDencun {
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
            withdrawals_root: rlp.val_at(16)?,
            blob_gas_used: rlp.val_at(17)?,
            excess_blob_gas: rlp.val_at(18)?,
            parent_beacon_block_root: rlp.val_at(19)?,
        })
    }
}

/// Verifies the hash of a Dencun block by comparing the computed hash with the provided hash.
///
/// This function verifies the integrity of a block by taking the block header from the database,
/// converting it to a `BlockHeaderDencun`, and encoding it using RLP. The resulting encoded data
/// is hashed using Keccak256, and the computed hash is compared to the provided `block_hash` to
/// ensure the block's authenticity.
///
/// # Arguments
///
/// - `block_hash`: A string containing the expected hash of the block.
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing raw block header data.
///
/// # Returns
///
/// A boolean indicating whether the computed block hash matches the provided block hash.
pub fn verify_hash_dencun(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderDencun::from_db_header(db_header);

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
    // use crate::block_header::BlockHeader as VerifiableBlockHeader;
    // use rlp::RlpStream;

    fn mock_block_header_dencun() -> BlockHeaderDencun {
        BlockHeaderDencun {
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
            withdrawals_root: H256::zero(),
            parent_beacon_block_root: H256::zero(),
            blob_gas_used: U256::zero(),
            excess_blob_gas: U256::zero(),
        }
    }

    #[test]
    fn test_encode_decode_dencun() {
        let header = mock_block_header_dencun();
        let encoded = header.rlp_encode();
        let decoded = BlockHeaderDencun::rlp_decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }
}
