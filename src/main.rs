mod block_header;
mod eras;
mod rpc_client;

use eras::{
    verify_hash_genesis_to_london, verify_hash_london_to_paris, verify_hash_paris_to_shanghai,
    verify_hash_shanghai_to_cancun,
};
use rpc_client::fetch_block_header;

#[tokio::main]
async fn main() {
    let rpc_url = "https://mainnet.infura.io/v3/25371764a3e44191b39d3b3b98a8c55d";
    let block_number = "0xC5D488"; // Replace with the actual block number in hex

    let block_number_dec = u64::from_str_radix(&block_number[2..], 16).unwrap();

    if (12965000..=15537393).contains(&block_number_dec) {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderLondonToParis>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_london_to_paris(block_hash, rpc_header);
    } else if (15537394..=17034869).contains(&block_number_dec) {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderParisToShanghai>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_paris_to_shanghai(block_hash, rpc_header);
    } else if block_number_dec > 17034869 {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderShanghaiToCancun>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_shanghai_to_cancun(block_hash, rpc_header);
    } else if block_number_dec <= 12964999 {
        let (block_hash, rpc_header) =
            fetch_block_header::<eras::RpcBlockHeaderGenesisToLondon>(rpc_url, block_number)
                .await
                .unwrap();
        verify_hash_genesis_to_london(block_hash, rpc_header);
    } else {
        println!("Block number is out of the supported range.");
    }
}
