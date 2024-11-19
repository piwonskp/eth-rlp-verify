pub trait BlockHeaderEncoder {
    fn rlp_encode(&self) -> Vec<u8>;
}

pub trait BlockHeaderTrait {
    fn compute_hash(&self) -> ethereum_types::H256;
    fn hex_to_fixed_array<const N: usize>(hex_str: &str) -> [u8; N];
}
