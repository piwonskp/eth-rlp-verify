use eth_rlp_verify::test_helpers::create_test_block_header_london;
use eth_rlp_verify::verify_block;
fn main() {
    let block_header = create_test_block_header_london();

    let block_hash = "0xa32d159805750cbe428b799a49b85dcb2300f61d806786f317260e721727d162";

    let is_valid = verify_block(12965001, block_header, block_hash);
    println!("London era block verification result: {}", is_valid);
}
