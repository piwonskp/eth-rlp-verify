use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use std::str::FromStr;
use tracing::{debug, info};

#[derive(Debug)]
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
    pub fn from_db_header(db_header: VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

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
}

// Implement the RLP encoding
impl BlockHeaderTrait for BlockHeaderGenesis {
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
}

// Verification logic
pub fn verify_hash_genesis_to_london(block_hash: String, db_header: VerifiableBlockHeader) -> bool {
    let header = BlockHeaderGenesis::from_db_header(db_header);

    // Log the RLP encoded data for debugging purposes
    let rlp_encoded = header.rlp_encode();
    debug!("RLP Encoded: {:?}", rlp_encoded);

    // Compute the block hash
    let computed_block_hash = header.compute_hash();
    info!("Computed Block Hash: {:?}", computed_block_hash);

    // Check if the computed hash matches the given block hash
    let is_valid = computed_block_hash == H256::from_str(&block_hash).unwrap();
    info!("Is the block hash valid? {}", is_valid);
    is_valid
}
