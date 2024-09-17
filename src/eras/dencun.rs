use crate::block_header::BlockHeader;
use crate::rpc_client::fetch_block_header;
use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use serde::Deserialize;
use std::str::FromStr;
use tracing::info;
use tracing::debug;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockHeaderDencun {
    pub parent_hash: String,
    pub sha3_uncles: String,
    pub miner: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub difficulty: String,
    pub number: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub timestamp: String,
    pub extra_data: String,
    pub mix_hash: String,
    pub nonce: String,
    pub base_fee_per_gas: String,
    pub withdrawals_root: String,
    pub parent_beacon_block_root: String, // New in Dencun
    pub blob_gas_used: String, // New in Dencun
    pub excess_blob_gas: String, // New in Dencun
}

#[derive(Debug)]
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
    pub blob_gas_used: U256, // New in Dencun
    pub excess_blob_gas: U256, // New in Dencun
}

impl BlockHeaderDencun {
    pub fn from_rpc(rpc_header: RpcBlockHeaderDencun) -> Self {
        let logs_bloom = <Self as BlockHeader>::hex_to_fixed_array::<256>(&rpc_header.logs_bloom);
        let nonce = <Self as BlockHeader>::hex_to_fixed_array::<8>(&rpc_header.nonce);

        BlockHeaderDencun {
            parent_hash: H256::from_str(&rpc_header.parent_hash).unwrap(),
            ommers_hash: H256::from_str(&rpc_header.sha3_uncles).unwrap(),
            beneficiary: H160::from_str(&rpc_header.miner).unwrap(),
            state_root: H256::from_str(&rpc_header.state_root).unwrap(),
            transactions_root: H256::from_str(&rpc_header.transactions_root).unwrap(),
            receipts_root: H256::from_str(&rpc_header.receipts_root).unwrap(),
            logs_bloom,
            difficulty: U256::from_str(&rpc_header.difficulty).unwrap(),
            number: U256::from_str(&rpc_header.number).unwrap(),
            gas_limit: U256::from_str(&rpc_header.gas_limit).unwrap(),
            gas_used: U256::from_str(&rpc_header.gas_used).unwrap(),
            timestamp: U256::from_str(&rpc_header.timestamp).unwrap(),
            extra_data: hex::decode(&rpc_header.extra_data[2..]).unwrap_or_default(),
            mix_hash: H256::from_str(&rpc_header.mix_hash).unwrap(),
            nonce,
            base_fee_per_gas: U256::from_str(&rpc_header.base_fee_per_gas).unwrap(),
            withdrawals_root: H256::from_str(&rpc_header.withdrawals_root).unwrap(),
            parent_beacon_block_root: H256::from_str(&rpc_header.parent_beacon_block_root).unwrap(),
            blob_gas_used: U256::from_str(&rpc_header.blob_gas_used).unwrap(),
            excess_blob_gas: U256::from_str(&rpc_header.excess_blob_gas).unwrap(),
        }
    }
}

impl BlockHeader for BlockHeaderDencun {
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
        stream.append(&self.blob_gas_used); // New in Dencun
        stream.append(&self.excess_blob_gas); // New in Dencun
        stream.append(&self.parent_beacon_block_root); // New in Dencun
        stream.out().to_vec()
    }
}

pub fn verify_hash_dencun(
    block_hash: String,
    rpc_header: RpcBlockHeaderDencun,
) {
    let header = BlockHeaderDencun::from_rpc(rpc_header);

    // Log the RLP encoded data for debugging purposes
    let rlp_encoded = header.rlp_encode();
    debug!("RLP Encoded: {:?}", rlp_encoded);

    // Compute the block hash
    let computed_block_hash = header.compute_hash();
    info!("Computed Block Hash: {:?}", computed_block_hash);

    // Check if the computed hash matches the given block hash
    let is_valid = computed_block_hash == H256::from_str(&block_hash).unwrap();
    info!("Is the block hash valid? {}", is_valid);
}

/// Helper function to verify blocks in the Dencun era
pub async fn verify_dencun(block_number: u64, rpc_url: String) {
    let block_number_hex = format!("0x{:X}", block_number);
    info!("Verifying block in the Dencun era");
    let (block_hash, rpc_header) =
        fetch_block_header::<RpcBlockHeaderDencun>(&rpc_url, &block_number_hex)
            .await
            .unwrap();
    verify_hash_dencun(block_hash, rpc_header);
}
