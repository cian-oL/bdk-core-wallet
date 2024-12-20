# BDK Core Wallet Demo

A Rust-based Bitcoin wallet implementation using the Bitcoin Development Kit (BDK) that demonstrates interaction with a Bitcoin Core node through RPC.

## Overview

This project showcases how to create a Bitcoin wallet using BDK that can interact with a Bitcoin Core node. It demonstrates various wallet operations including:

- Creating and managing wallets
- Generating addresses
- Sending and receiving transactions
- Working with PSBTs (Partially Signed Bitcoin Transactions)
- Blockchain synchronization

## Prerequisites

- Rust (latest stable version)
- Bitcoin Core node running in regtest mode
- Basic understanding of Bitcoin transactions and wallet operations

## Dependencies

- bdk v0.10.0 (with features: all-keys, key-value-db, rpc)
- anyhow v1.0.94
- dirs-next v2.0.0

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/bdk-core-wallet.git
cd bdk-core-wallet
```

2. Build the project:

```bash
cargo build
```

## Configuration

The application expects a Bitcoin Core node running locally with the following default configuration:

- RPC URL: http://127.0.0.1:18443
- Network: Regtest
- Username: admin
- Password: password

## Usage

1. Start your Bitcoin Core node in regtest mode.

2. Run the application:

```bash
cargo run
```

The application will:

1. Create a test wallet in Bitcoin Core
2. Generate initial blocks for testing
3. Create a BDK wallet with deterministic descriptors
4. Perform a test transaction
5. Sign and broadcast the transaction

## Features

- **Wallet Creation**: Generates deterministic wallets
- **Address Management**: Creates and manages addresses using descriptor-based wallet structure
- **Transaction Handling**: Demonstrates complete transaction lifecycle including:
  - Creating transactions
  - Signing with PSBTs
  - Broadcasting to the network
- **Blockchain Synchronization**: Syncs wallet state with the Bitcoin network (regtest)
- **Secure Key Management**: Uses BIP32 derivation paths for key management

## Project Structure

- `src/main.rs`: Main application logic and wallet operations
- `Cargo.toml`: Project dependencies and configuration

## License

This project is open source and is intended for educational purposes.

## Acknowledgments

- Bitcoin Development Kit (BDK) team
- Bitcoin Core developers
- Rajarshi Maitra

## Resources

- [Bitcoin Development Kit Documentation](https://bitcoindevkit.org)
- [BDK Core RPC Demo Blog Post](https://bitcoindevkit.org/blog/bitcoin-core-rpc-demo/)
- [Bitcoin Core RPC Documentation](https://developer.bitcoin.org/reference/rpc/)

## Disclaimer

This Readme was generated in first draft by Windsurf (https://codeium.com/windsurf)
