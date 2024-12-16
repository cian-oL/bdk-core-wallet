use anyhow::Result;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath, KeySource};
use bdk::bitcoin::Amount;
use bdk::bitcoin::Network;
use bdk::bitcoincore_rpc::{Auth as rpc_auth, Client, RpcApi};
use bdk::blockchain::rpc::{wallet_name_from_descriptor, Auth, RpcBlockchain, RpcConfig};
use bdk::blockchain::{ConfigurableBlockchain, NoopProgress};
use bdk::keys::bip39::{Language, Mnemonic, MnemonicType};
use bdk::keys::DescriptorKey::Secret;
use bdk::keys::{DerivableKey, DescriptorKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::miniscript::Segwitv0;
use bdk::sled;
use bdk::wallet::{signer::SignOptions, AddressIndex};
use bdk::Wallet;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

/// generates and returns a (receiver, change) pair of descriptors
fn get_descriptors() -> Result<(String, String)> {
    let secp = Secp256k1::new();
    let password = Some("password".to_string());

    // generate private key from mnemonic and password
    let mnemonic: GeneratedKey<_, Segwitv0> =
        Mnemonic::generate((MnemonicType::Words12, Language::English)).unwrap();
    let mnemonic = mnemonic.into_key();
    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Regtest).unwrap();

    Ok(("Ok".to_string(), "Ok".to_string()))
}
