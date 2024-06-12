mod genesis_to_london;
mod london_to_paris;
mod paris_to_shanghai;
mod shanghai_to_cancun;

pub use genesis_to_london::{verify_hash_genesis_to_london, RpcBlockHeaderGenesisToLondon};
pub use london_to_paris::{verify_hash_london_to_paris, RpcBlockHeaderLondonToParis};
pub use paris_to_shanghai::{verify_hash_paris_to_shanghai, RpcBlockHeaderParisToShanghai};
pub use shanghai_to_cancun::{verify_hash_shanghai_to_cancun, RpcBlockHeaderShanghaiToCancun};
