use eth_rlp_verify::block_header::BlockHeader as VerifiableBlockHeader;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = VerifiableBlockHeader {
        parent_hash: Some("0x3de6bb3849a138e6ab0b83a3a00dc7433f1e83f7fd488e4bba78f2fe2631a633".to_string()),
        block_hash: "0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string(),
        number: 12965001, // Correct decimal representation of "0xc5d489"
        gas_limit: 30029122, // Correct decimal representation of "0x1c9c2b6"
        gas_used: 30025257, // Correct decimal representation of "0x1c98978"
        nonce: "0xb223da049adf2216".to_string(),
        transaction_root: Some("0xdfcb68d3a3c41096f4a77569db7956e0a0e750fad185948e54789ea0e51779cb".to_string()),
        receipts_root: Some("0x8a8865cd785e2e9dfce7da83aca010b10b9af2abbd367114b236f149534c821d".to_string()),
        state_root: Some("0x41cf6e8e60fd087d2b00360dc29e5bfb21959bce1f4c242fd1ad7c4da968eb87".to_string()),
        base_fee_per_gas: Some("0x3b9aca00".to_string()),
        miner: Some("0x7777788200b672a42421017f65ede4fc759564c8".to_string()),
        logs_bloom: Some("0x24e74ad77d9a2b27bdb8f6d6f7f1cffdd8cfb47fdebd433f011f7dfcfbb7db638fadd5ff66ed134ede2879ce61149797fbcdf7b74f6b7de153ec61bdaffeeb7b59c3ed771a2fe9eaed8ac70e335e63ff2bfe239eaff8f94ca642fdf7ee5537965be99a440f53d2ce057dbf9932be9a7b9a82ffdffe4eeee1a66c4cfb99fe4540fbff936f97dde9f6bfd9f8cefda2fc174d23dfdb7d6f7dfef5f754fe6a7eec92efdbff779b5feff3beafebd7fd6e973afebe4f5d86f3aafb1f73bf1e1d0cdd796d89827edeffe8fb6ae6d7bf639ec5f5ff4c32f31f6b525b676c7cdf5e5c75bfd5b7bd1928b6f43aac7fa0f6336576e5f7b7dfb9e8ebbe6f6efe2f9dfe8b3f56".to_string()),
        difficulty: Some("0x1b81c1fe05b218".to_string()),
        total_difficulty: Some("0x608af5dd578251af429".to_string()),
        sha3_uncles: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        timestamp: Some("1628166822".to_string()), // Correct decimal representation of "0x610bdab3"
        extra_data: Some("0x68747470733a2f2f7777772e6b7279707465782e6f7267".to_string()),
        mix_hash: Some("0x9620b46a81a4795cf4449d48e3270419f58b09293a5421205f88179b563f815a".to_string()),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    };

    let block_hash = "0xa32d159805750cbe428b799a49b85dcb2300f61d806786f317260e721727d162";

    let is_valid = verify_block(12965001, block_header, block_hash);
    println!("London era block verification result: {}", is_valid);
}
