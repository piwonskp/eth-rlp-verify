/// The last block number of the Genesis to London era.
///
/// The Genesis to London era encompasses the initial phases of the Ethereum blockchain, starting from the Genesis block
/// (block 0) and ending with the final block before the London upgrade. This period includes several key Ethereum upgrades
/// such as Frontier, Homestead, and Byzantium, and culminates with the introduction of the London hard fork, which
/// introduced EIP-1559 and a new gas fee model.
///
/// # Value
/// - `12,964,999` is the block number marking the last block before the London upgrade.
pub const GENESIS_END: u64 = 12_964_999;

/// The first block number of the London to Paris era.
///
/// The London to Paris era begins with the London upgrade at block `12,965,000`, which introduced the new EIP-1559 gas fee mechanism.
/// This period also encompasses the move towards Ethereum 2.0 and the transition from proof-of-work (PoW) to proof-of-stake (PoS).
///
/// # Value
/// - `12,965,000` is the block number where the London upgrade starts.
pub const LONDON_START: u64 = 12_965_000;

/// The last block number of the London to Paris era.
///
/// This constant marks the final block of the London era and the transition to the Paris era, which corresponds to
/// Ethereum's "Merge" where it transitioned from proof-of-work (PoW) to proof-of-stake (PoS).
///
/// # Value
/// - `15,537,393` is the block number marking the end of the London era, right before the Paris upgrade.
pub const LONDON_END: u64 = 15_537_393;

/// The first block number of the Paris to Shanghai era.
///
/// The Paris to Shanghai era begins with the Paris upgrade, also known as "The Merge," at block `15,537,394`.
/// The Merge marks Ethereum's transition from proof-of-work to proof-of-stake, an event that significantly changed
/// how the Ethereum network secures itself and validates transactions.
///
/// # Value
/// - `15,537,394` is the block number where the Paris upgrade begins.
pub const PARIS_START: u64 = 15_537_394;

/// The last block number of the Paris to Shanghai era.
///
/// This constant defines the end of the Paris era and the beginning of the Shapella upgrade (Shanghai + Capella).
/// It includes the transition from proof-of-stake and other key improvements to Ethereum's protocol.
///
/// # Value
/// - `17,034,869` is the block number marking the end of the Paris era.
pub const PARIS_END: u64 = 17_034_869;

/// The first block number of the Shapella era.
///
/// The Shapella era begins with the Shapella upgrade at block `17,034,870`. The Shapella upgrade consists of both the
/// Shanghai and Capella updates and introduces major improvements, including enabling staked ETH withdrawals from
/// the Ethereum beacon chain.
///
/// # Value
/// - `17,034,870` is the block number where the Shapella upgrade starts.
pub const SHAPELLA_START: u64 = 17_034_870;

/// The last block number of the Shapella era.
///
/// This constant marks the last block of the Shapella era, a period that includes staked ETH withdrawals and other key improvements.
/// The Shapella era concludes just before the start of the Dencun upgrade.
///
/// # Value
/// - `19,426,586` is the block number marking the end of the Shapella era.
pub const SHAPELLA_END: u64 = 19_426_586;

/// The first block number of the Dencun era.
///
/// The Dencun era begins at block `19,426,587` with the Dencun upgrade, which introduces key improvements to
/// Ethereum's scalability, gas fee adjustments, and other protocol changes. This era continues the evolution of
/// Ethereum after Shapella, with additional updates to the proof-of-stake consensus mechanism.
///
/// # Value
/// - `19,426,587` is the block number where the Dencun upgrade begins.
pub const DENCUN_START: u64 = 19_426_587;
