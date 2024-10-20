# `eth-rlp-verify`

`eth-rlp-verify` is a Rust crate that provides functionality for verifying Ethereum block headers across multiple eras, such as **London**, **Paris (The Merge)**, **Shapella**, and **Dencun**. The library automatically determines the correct Ethereum era based on the block number and validates the block header by computing its hash and comparing it to the expected block hash.

## Table of Contents

- [`eth-rlp-verify`](#eth-rlp-verify)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Fetching Block Headers](#fetching-block-headers)
    - [Verifying Across Eras](#verifying-across-eras)
  - [Ethereum Eras](#ethereum-eras)
  - [Modules](#modules)
    - [`block_header`](#block_header)
    - [`constants`](#constants)
    - [`eras`](#eras)
  - [Contributing](#contributing)
    - [Adding Future Support](#adding-future-support)
  - [License](#license)

## Overview

Ethereum has undergone several significant upgrades, each introducing changes to the block header structure and consensus mechanism. The `eth-rlp-verify` crate provides a solution for verifying block headers in different eras of Ethereum. It supports Recursive Length Prefix (RLP) encoding, Keccak256 hashing, and era-specific validation logic to ensure the authenticity of Ethereum block headers.

## Features

- **Era-based block header verification**: Automatically detects the Ethereum era based on the block number and applies the appropriate validation logic.
- **RLP encoding**: Uses RLP to efficiently encode block headers, a key part of Ethereum's serialization.
- **Supports multiple Ethereum upgrades**: Handles block header verification for the **Genesis**, **London**, **Paris**, **Shapella**, and **Dencun** eras.
- **Hash verification**: Verifies block headers by computing their Keccak256 hash and comparing it with the expected hash.
- **Extensible design**: Prepared for easy integration of future Ethereum upgrades.

## Installation

To use `eth-rlp-verify` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
eth-rlp-verify = "0.1.0"
```

Then, import the necessary modules:

```rust
use eth_rlp_verify::verify_block;
use eth_rlp_verify::block_header::BlockHeader;
```

## Usage


### Fetching Block Headers

You must provide a `BlockHeader` struct containing the necessary fields (such as `parent_hash`, `state_root`, `transactions_root`, etc.) to verify a block. This struct will vary slightly depending on the Ethereum era.

### Verifying Across Eras

The `eth_rlp_verify` crate automatically determines the correct era based on the block number and applies the appropriate verification logic for that era:

```rust
let block_number = 17_034_870; // Shapella era
let is_valid = verify_block(block_number, block_header, block_hash);
```

## Ethereum Eras

`eth-rlp-verify` supports the following Ethereum eras:

- **Genesis to London**: The era from the Genesis block to the London upgrade, including early Ethereum developments and the introduction of EIP-1559.
- **London to Paris (The Merge)**: The era spanning from the London upgrade to the Paris upgrade (The Merge), where Ethereum transitions from proof-of-work (PoW) to proof-of-stake (PoS).
- **Paris to Shapella**: The post-Merge era from Paris to the Shapella upgrade, which enables staked ETH withdrawals and introduces additional improvements.
- **Shapella to Dencun**: Starting with Shapella and continuing into the Dencun upgrade, introducing new features like blob transactions.

## Modules

### `block_header`
Contains the `BlockHeader` struct and related traits for managing and encoding block header data across different Ethereum eras.

### `constants`
Defines constants for block ranges corresponding to different Ethereum eras. Example:

```rust
pub const GENESIS_END: u64 = 12_964_999;
pub const LONDON_START: u64 = 12_965_000;
pub const LONDON_END: u64 = 15_537_393;
pub const PARIS_START: u64 = 15_537_394;
pub const SHAPELLA_START: u64 = 17_034_870;
pub const DENCUN_START: u64 = 19_426_587;
```

### `eras`
Handles the logic for determining which Ethereum era a block belongs to based on the block number. The `determine_era` function returns the appropriate block header verification function for that era.

```rust
pub fn determine_era(block_number: u64) -> Option<fn(String, VerifiableBlockHeader) -> bool>;
```

## Contributing

We welcome contributions! If you’d like to improve or extend the `eth-rlp-verify` crate, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Open a pull request and describe your changes.

### Adding Future Support

If you're interested in adding support for future Ethereum upgrades, check out the `eras` module, where the logic for handling block verification by era is implemented. You can extend this module to include new Ethereum upgrades as they are introduced.

## License

This project is licensed under the MIT License. For more information, see the [LICENSE](LICENSE) file.

---

Feel free to adjust the README as you see fit!Test workflow trigger

Test workflow trigger