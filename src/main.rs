mod block_header;
mod constants;
mod eras;
mod rpc_client;

use constants::{
    GENESIS_TO_LONDON_END, LONDON_TO_PARIS_END, LONDON_TO_PARIS_START, PARIS_TO_SHANGHAI_END,
    PARIS_TO_SHANGHAI_START,
};
use eras::{
    verify_hash_genesis_to_london, verify_hash_london_to_paris, verify_hash_paris_to_shanghai,
    verify_hash_shanghai_to_cancun,
};
use rpc_client::fetch_block_header;

/// Main function to verify block headers against their hashes based on block number ranges.
///
/// # Arguments
///
/// * `rpc_url` - The URL of the Ethereum RPC endpoint.
/// * `block_number` - The block number in hexadecimal format.
#[tokio::main]
async fn main() {
    let rpc_url = "https://mainnet.infura.io/v3/25371764a3e44191b39d3b3b98a8c55d";
    let block_number = "0xED14F2"; // Replace with the actual block number in hex

    let block_number_dec = u64::from_str_radix(&block_number[2..], 16).unwrap();

    if (LONDON_TO_PARIS_START..=LONDON_TO_PARIS_END).contains(&block_number_dec) {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderLondonToParis>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_london_to_paris(block_hash, rpc_header);
    } else if (PARIS_TO_SHANGHAI_START..=PARIS_TO_SHANGHAI_END).contains(&block_number_dec) {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderParisToShanghai>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_paris_to_shanghai(block_hash, rpc_header);
    } else if block_number_dec > PARIS_TO_SHANGHAI_END {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderShanghaiToCancun>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_shanghai_to_cancun(block_hash, rpc_header);
    } else if block_number_dec <= GENESIS_TO_LONDON_END {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderGenesisToLondon>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_genesis_to_london(block_hash, rpc_header);
    } else {
        println!("Block number is out of the supported range.");
    }
}
