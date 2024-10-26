use eth_rlp_verify::test_helpers::create_test_block_header_paris;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = create_test_block_header_paris();

    let block_hash = "0xe37e1a183a3d1c7234d090bfb7196081635919c26f2e65c67c106513158a7db4";

    let is_valid = verify_block(15537401, block_header, block_hash);
    println!("Paris era block verification result: {}", is_valid);
}
