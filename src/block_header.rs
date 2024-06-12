use ethereum_types::H256;
use sha3::{Digest, Keccak256};

pub trait BlockHeader {
    fn rlp_encode(&self) -> Vec<u8>;

    fn compute_hash(&self) -> H256 {
        let rlp_encoded = self.rlp_encode();
        let hash = Keccak256::digest(rlp_encoded);
        H256::from_slice(&hash)
    }

    fn hex_to_fixed_array<const N: usize>(hex_str: &str) -> [u8; N] {
        let bytes = hex::decode(&hex_str[2..]).unwrap();
        let mut array = [0u8; N];
        let len = bytes.len();
        if len != N {
            panic!("Invalid input length: expected {}, got {}", N, len);
        }
        array.copy_from_slice(&bytes);
        array
    }
}
