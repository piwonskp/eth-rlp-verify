mod dencun;
mod genesis;
mod london;
mod paris;
mod shapella;

use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait};
use crate::constants::{
    DENCUN_START, GENESIS_END, LONDON_END, LONDON_START, PARIS_END, PARIS_START, SHAPELLA_END,
    SHAPELLA_START,
};

// Re-export each era's verification function to make them accessible at the module level.
pub use dencun::verify_hash_dencun;
pub use genesis::verify_hash_genesis;
pub use london::verify_hash_london;
pub use paris::verify_hash_paris;
pub use shapella::verify_hash_shapella;

type DecoderFn = fn(&[u8]) -> Result<VerifiableBlockHeader, eyre::Report>;

/// Determines the correct Ethereum era based on the block number and returns the corresponding
/// hash verification function for that era.
///
/// Ethereum undergoes upgrades (also known as "eras") that introduce significant changes to the protocol.
/// Each era has its own block header format, which requires specialized verification logic. This function
/// helps determine which era a block belongs to based on the block number and returns the appropriate
/// function to verify the block's hash.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the number of the block whose era needs to be determined.
///
/// # Returns
///
/// An `Option<fn(String, VerifiableBlockHeader) -> bool>`, which is:
/// - `Some(verification_function)` if the block number corresponds to a recognized era, where the
///   returned function can be used to verify the block's hash.
/// - `None` if the block number does not match any known era.
///
/// # Supported Eras and Their Block Ranges:
///
/// - **London:** The era between `LONDON_START` and `LONDON_END`, inclusive. This upgrade introduced EIP-1559, changing the gas fee model.
/// - **Paris:** The era between `PARIS_START` and `PARIS_END`, representing Ethereum's merge from proof-of-work (PoW) to proof-of-stake (PoS).
/// - **Shapella:** The era between `SHAPELLA_START` and `SHAPELLA_END`, introducing further changes to Ethereum's staking and withdrawal mechanisms.
/// - **Dencun:** Blocks from `DENCUN_START` onwards, which represent the upcoming Dencun upgrade.
/// - **Genesis:** The initial blocks from block 0 up to `GENESIS_END`, covering Ethereum's early history.
///
/// # Notes
///
/// - If the block number falls outside the recognized eras, this function will return `None`.
pub fn determine_era(block_number: u64) -> Option<fn(String, VerifiableBlockHeader) -> bool> {
    if (LONDON_START..=LONDON_END).contains(&block_number) {
        Some(verify_hash_london)
    } else if (PARIS_START..=PARIS_END).contains(&block_number) {
        Some(verify_hash_paris)
    } else if (SHAPELLA_START..=SHAPELLA_END).contains(&block_number) {
        Some(verify_hash_shapella)
    } else if block_number >= DENCUN_START {
        Some(verify_hash_dencun)
    } else if block_number <= GENESIS_END {
        Some(verify_hash_genesis)
    } else {
        None
    }
}

pub fn determine_era_encoder(block_number: u64) -> Option<fn(VerifiableBlockHeader) -> Vec<u8>> {
    if (LONDON_START..=LONDON_END).contains(&block_number) {
        Some(|header| london::BlockHeaderLondon::from_db_header(header).rlp_encode())
    } else if (PARIS_START..=PARIS_END).contains(&block_number) {
        Some(|header| paris::BlockHeaderParis::from_db_header(header).rlp_encode())
    } else if (SHAPELLA_START..=SHAPELLA_END).contains(&block_number) {
        Some(|header| shapella::BlockHeaderShapella::from_db_header(header).rlp_encode())
    } else if block_number >= DENCUN_START {
        Some(|header| dencun::BlockHeaderDencun::from_db_header(header).rlp_encode())
    } else if block_number <= GENESIS_END {
        Some(|header| genesis::BlockHeaderGenesis::from_db_header(header).rlp_encode())
    } else {
        None
    }
}

pub fn determine_era_decoder(block_number: u64) -> Option<DecoderFn> {
    if block_number <= GENESIS_END {
        Some(|data| genesis::BlockHeaderGenesis::rlp_decode(data).map(|h| h.into_verifiable()))
    } else if (LONDON_START..=LONDON_END).contains(&block_number) {
        Some(|data| london::BlockHeaderLondon::rlp_decode(data).map(|h| h.into_verifiable()))
    } else if (PARIS_START..=PARIS_END).contains(&block_number) {
        Some(|data| paris::BlockHeaderParis::rlp_decode(data).map(|h| h.into_verifiable()))
    } else if (SHAPELLA_START..=SHAPELLA_END).contains(&block_number) {
        Some(|data| shapella::BlockHeaderShapella::rlp_decode(data).map(|h| h.into_verifiable()))
    } else if block_number >= DENCUN_START {
        Some(|data| dencun::BlockHeaderDencun::rlp_decode(data).map(|h| h.into_verifiable()))
    } else {
        None
    }
}
