use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use eyre::Result;
use rlp::{Rlp, RlpStream};
use std::str::FromStr;
use tracing::debug;

/// Represents a Shapella Ethereum block header.
///
/// Shapella is an upgrade to Ethereum's consensus mechanism, and this struct holds
/// all the key fields of a block header, including cryptographic hashes (e.g., parent hash,
/// state root), metadata (e.g., gas usage, timestamp), and other data required for verification.
///
/// The block header is integral to the structure of the blockchain, helping to link blocks
/// together and validate their authenticity through the consensus algorithm.
///
/// # Fields
///
/// - `parent_hash`: Hash of the parent block, which is the previous block in the chain.
/// - `ommers_hash`: The hash of ommer (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner or validator who created this block.
/// - `state_root`: The root hash of the state trie after the block is processed.
/// - `transactions_root`: The root hash of the Merkle tree of transactions.
/// - `receipts_root`: The root hash of the Merkle tree of transaction receipts.
/// - `logs_bloom`: A bloom filter used for quick searching and filtering of logs.
/// - `difficulty`: The difficulty of mining this block, related to proof of work.
/// - `number`: The block number, representing its position in the blockchain.
/// - `gas_limit`: The maximum amount of gas allowed in this block.
/// - `gas_used`: The total amount of gas used by all transactions in this block.
/// - `timestamp`: The timestamp of when the block was mined.
/// - `extra_data`: Extra information added by the miner, up to 32 bytes.
/// - `mix_hash`: A hash used in proof-of-work validation to ensure the block was mined correctly.
/// - `nonce`: A 64-bit proof-of-work nonce used to validate the block's difficulty target.
/// - `base_fee_per_gas`: The minimum gas price per unit for this block, part of EIP-1559.
/// - `withdrawals_root`: The root hash of withdrawals in the block, introduced in Shapella.
#[derive(Debug, PartialEq)]
pub struct BlockHeaderShapella {
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
}

impl BlockHeaderShapella {
    /// Converts a `VerifiableBlockHeader` (from the database) into a `BlockHeaderShapella`.
    ///
    /// This method takes a block header from the database, represented by the `VerifiableBlockHeader` struct,
    /// and transforms it into a `BlockHeaderShapella` struct with all the appropriate fields parsed and populated.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing various fields that
    ///   are mapped into the Shapella block header structure.
    ///
    /// # Returns
    ///
    /// Returns a `BlockHeaderShapella` instance with parsed values from the database header.
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderShapella {
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
        }
    }

    /// Converts a `BlockHeaderShapella` into a common `VerifiableBlockHeader`.
    ///
    /// This method ensures that the Shapella-specific block header structure is converted into the
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
            totaldifficulty: None, // Not applicable for Shapella.
            number: self.number.as_u64() as i64,
            gas_limit: self.gas_limit.as_u64() as i64,
            gas_used: self.gas_used.as_u64() as i64,
            timestamp: Some(self.timestamp.to_string()),
            extra_data: Some(hex::encode(self.extra_data)),
            mix_hash: Some(self.mix_hash.to_string()),
            nonce: hex::encode(self.nonce),
            base_fee_per_gas: Some(self.base_fee_per_gas.to_string()),
            withdrawals_root: Some(self.withdrawals_root.to_string()),
            blob_gas_used: None,
            excess_blob_gas: None,
            parent_beacon_block_root: None,
            sha3_uncles: None, // Not applicable for Shapella.
        }
    }
}

/// Implements the `BlockHeaderTrait` for the `BlockHeaderShapella` type.
///
/// This provides the ability to encode the block header using RLP encoding, which is a compact,
/// efficient encoding used in Ethereum for serializing block headers and other objects.
///
/// This method also includes the computation of the block hash, which is crucial for
/// validating the integrity and authenticity of blocks in the Ethereum blockchain.
impl BlockHeaderTrait for BlockHeaderShapella {
    /// RLP encodes the block header, producing a vector of bytes.
    ///
    /// This method encodes all 17 fields of the Shapella block header, following the Ethereum
    /// specification for block headers. RLP encoding is used to ensure compact storage and transmission
    /// of the block data, which is then used for hashing, verification, and blockchain consensus.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(17); // 17 fields in Shapella block header
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
        stream.out().to_vec()
    }

    fn rlp_decode(data: &[u8]) -> Result<Self> {
        let rlp = Rlp::new(data);
        Ok(BlockHeaderShapella {
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
        })
    }
}

/// Verifies the integrity of a Shapella block hash by comparing it with a computed hash.
///
/// This function computes the block hash using the `BlockHeaderShapella` and compares it to the provided `block_hash`.
/// It logs the RLP-encoded block header and the computed block hash for debugging purposes and checks
/// whether the computed hash matches the given hash.
///
/// # Arguments
///
/// - `block_hash`: A string containing the hash of the block (in hexadecimal format).
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, which will be used to construct
///   the `BlockHeaderShapella` and compute the hash.
///
/// # Returns
///
/// A boolean indicating whether the computed block hash matches the provided `block_hash`.
pub fn verify_hash_shapella(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderShapella::from_db_header(db_header);

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

    fn mock_block_header_shapella() -> BlockHeaderShapella {
        BlockHeaderShapella {
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
        }
    }

    #[test]
    fn test_encode_decode_shapella() {
        let header = mock_block_header_shapella();
        let encoded = header.rlp_encode();
        let decoded = BlockHeaderShapella::rlp_decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }
}
