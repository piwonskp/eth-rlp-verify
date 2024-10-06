use eth_rlp_verify::block_header::BlockHeader as VerifiableBlockHeader;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = VerifiableBlockHeader {
        block_hash: "0xa49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e051"
            .to_string(),
        number: 15537401,
        gas_limit: 30000000,
        gas_used: 23920666,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x72c274792a9158b66388a5b0a86c2602876f36a5af00970f730a86f17902fa7a".to_string(),
        ),
        receipts_root: Some(
            "0xf456de14cba28b2b4a893c59fb6b8b8ebd0b77e6b642f207472204420db4ab53".to_string(),
        ),
        state_root: Some(
            "0x88a254cb6a13656d7a2a3a6afec25cb321be15b269e45d4e3887c4ea257f97f2".to_string(),
        ),
        base_fee_per_gas: Some("0x3b9aca00".to_string()),
        parent_hash: Some(
            "0x38f345f321d623c5d9b9acbd357d017c2f741349b42d737283c6d4bcd6942eb5".to_string(),
        ),
        miner: Some("0xea674fdde714fd979de3edf0f56aa9716b898ec8".to_string()),
        logs_bloom: Some("0xbb6fd21...".to_string()),
        difficulty: Some("0x0".to_string()),
        total_difficulty: Some("0xc8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("1633028945".to_string()),
        extra_data: Some("0x65746865726d696e652e696f".to_string()),
        mix_hash: Some(
            "0x234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        ),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    };

    let block_hash = "0xa49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e051";

    let is_valid = verify_block(15537401, block_header, block_hash);
    println!("Paris era block verification result: {}", is_valid);
}
