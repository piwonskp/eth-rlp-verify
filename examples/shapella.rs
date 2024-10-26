use eth_rlp_verify::test_helpers::create_test_block_header_shapella;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = create_test_block_header_shapella();

    let block_hash = "0x17cf53189035bbae5bce5c844355badd701aa9d2dd4b4f5ab1f9f0e8dd9fea5b";

    let is_valid = verify_block(17034871, block_header, block_hash);
    println!("Shapella era block verification result: {}", is_valid);
}
