use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use eyre::Result;
use rlp::{Rlp, RlpStream};
use std::str::FromStr;
use tracing::debug;

/// Represents an Ethereum block header for the Genesis era.
///
/// The Genesis era refers to the early blocks in the Ethereum blockchain, starting from the Genesis block
/// and continuing through the initial blocks before major upgrades. This struct contains the fields necessary
/// for verifying blocks from the Genesis era, including cryptographic data, gas usage, and other metadata
/// required to validate and authenticate these early blocks.
///
/// # Fields
///
/// - `parent_hash`: The hash of the parent block that links this block to the blockchain.
/// - `ommers_hash`: The hash of the ommers (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner who produced this block.
/// - `state_root`: The root of the state trie after the block has been processed.
/// - `transactions_root`: The Merkle root of the transactions included in the block.
/// - `receipts_root`: The Merkle root of the receipts for transactions included in the block.
/// - `logs_bloom`: A 256-byte bloom filter for fast searching and filtering of logs.
/// - `difficulty`: The difficulty level required to mine the block in the proof-of-work algorithm.
/// - `number`: The block number, which indicates its position in the blockchain.
/// - `gas_limit`: The maximum amount of gas that transactions within the block can consume.
/// - `gas_used`: The actual gas consumed by the transactions in this block.
/// - `timestamp`: The time at which the block was mined.
/// - `extra_data`: Additional data added by the miner, up to 32 bytes.
/// - `mix_hash`: A proof-of-work hash used to verify the mining process.
/// - `nonce`: A 64-bit proof-of-work nonce used to verify the mining result.
#[derive(Debug, PartialEq)]
pub struct BlockHeaderGenesis {
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
}

impl BlockHeaderGenesis {
    /// Converts a `VerifiableBlockHeader` fetched from the database into a `BlockHeaderGenesis`.
    ///
    /// This method takes a database block header and converts it into the appropriate `BlockHeaderGenesis` structure
    /// by parsing and populating all necessary fields such as `parent_hash`, `ommers_hash`, and `logs_bloom`.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing various fields in string format.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderGenesis` instance containing the parsed and validated block header data.
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.clone().unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce.clone());

        BlockHeaderGenesis {
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
        }
    }

    pub fn into_verifiable(self) -> VerifiableBlockHeader {
        VerifiableBlockHeader {
            block_hash: "".to_string(),
            parent_hash: Some(self.parent_hash.to_string()),
            ommers_hash: Some(self.ommers_hash.to_string()),
            miner: Some(self.beneficiary.to_string()),
            state_root: Some(self.state_root.to_string()),
            transaction_root: Some(self.transactions_root.to_string()),
            receipts_root: Some(self.receipts_root.to_string()),
            logs_bloom: Some(hex::encode(self.logs_bloom)),
            difficulty: Some(self.difficulty.to_string()),
            totaldifficulty: None,
            number: self.number.as_u64() as i64,
            gas_limit: self.gas_limit.as_u64() as i64,
            gas_used: self.gas_used.as_u64() as i64,
            timestamp: Some(self.timestamp.to_string()),
            extra_data: Some(hex::encode(self.extra_data)),
            mix_hash: Some(self.mix_hash.to_string()),
            nonce: hex::encode(self.nonce),
            base_fee_per_gas: None,
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
            parent_beacon_block_root: None,
            sha3_uncles: None,
        }
    }
}

/// Implements the `BlockHeaderTrait` for `BlockHeaderGenesis`.
///
/// This implementation provides RLP encoding for the Genesis block header, which is critical for compact
/// storage and transmission of the block header in Ethereum. The RLP encoding is also used during the process
/// of verifying the block header by computing its hash.
impl BlockHeaderTrait for BlockHeaderGenesis {
    /// RLP encodes the Genesis block header, returning a vector of bytes.
    ///
    /// This function encodes all 15 fields of the Genesis block header using Ethereum's RLP (Recursive Length Prefix) format,
    /// which is essential for serialization, verification, and block consistency checks.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(15); // 15 fields in Genesis block header
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
        stream.out().to_vec()
    }

    /// Decodes an RLP-encoded byte slice into a `BlockHeaderGenesis`.
    ///
    /// This function decodes the byte slice into the 15 fields of the Genesis block header.
    ///
    /// # Arguments
    /// - `data`: A byte slice containing the RLP-encoded data.
    ///
    /// # Returns
    /// - A `Result<Self>` containing the decoded block header or an error if decoding fails.
    fn rlp_decode(data: &[u8]) -> Result<Self> {
        let rlp = Rlp::new(data);
        Ok(BlockHeaderGenesis {
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
        })
    }
}

/// Verifies the hash of a Genesis era block by comparing the computed hash to the provided hash.
///
/// This function verifies the integrity of a block from the Genesis era by taking a block header from
/// the database, converting it to a `BlockHeaderGenesis`, and encoding it using RLP. The computed hash
/// is then compared to the provided block hash to ensure correctness.
///
/// # Arguments
///
/// - `block_hash`: The expected hash of the block as a hexadecimal string.
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw block header data.
///
/// # Returns
///
/// A boolean value indicating whether the computed block hash matches the provided block hash.
pub fn verify_hash_genesis(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderGenesis::from_db_header(db_header);

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

    fn mock_block_header_genesis() -> BlockHeaderGenesis {
        BlockHeaderGenesis {
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
        }
    }

    #[test]
    fn test_encode_decode_genesis() {
        let header = mock_block_header_genesis();
        let encoded = header.rlp_encode();
        let decoded = BlockHeaderGenesis::rlp_decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }
}
