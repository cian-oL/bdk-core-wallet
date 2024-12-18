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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create rpc interface
    let rpc_auth = rpc_auth::UserPass("admin".to_string(), "password".to_string());
    let core_rpc = Client::new("http://127.0.0.1:18443/wallet/test".to_string(), rpc_auth)?;
    println!("{:#?}", core_rpc.get_blockchain_info()?);

    // create test wallet and coins (coinbase maturity needs 100 blocks)
    core_rpc.create_wallet("test", None, None, None, None)?;
    let core_address = core_rpc.get_new_address(None, None)?;
    core_rpc.generate_to_address(101, &core_address)?; // 101 blocks for > maturity
    let core_balance = core_rpc.get_balance(None, None)?;

    println!("core balance: {:#?}", core_balance);
    Ok(())
}

/// generates and returns a (receiver, change) pair of descriptors
fn get_descriptors() -> Result<(String, String)> {
    let secp = Secp256k1::new();
    let password = Some("password".to_string());

    // generate master private key from mnemonic and password
    let mnemonic: GeneratedKey<_, Segwitv0> =
        Mnemonic::generate((MnemonicType::Words12, Language::English)).unwrap();
    let mnemonic = mnemonic.into_key();
    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Regtest).unwrap();

    // derive receiver "m/84h/1h/0h/0" and change "m/84h/1h/0h/1" keys
    let mut keys = Vec::new();

    for path in ["m/84h/1h/0h/0", "m/84h/1h/0h/1"] {
        let deriv_path = DerivationPath::from_str(path).unwrap();
        let derived_xprv = &xprv.derive_priv(&secp, &deriv_path).unwrap();
        let origin = (xprv.fingerprint(&secp), deriv_path);
        let derived_xprv_desc_key: DescriptorKey<Segwitv0> = derived_xprv
            .into_descriptor_key(Some(origin), DerivationPath::default())
            .unwrap();

        if let Secret(key, _, _) = derived_xprv_desc_key {
            let mut desc = "wpkh".to_string();
            desc.push_str(&key.to_string());
            desc.push_str(")");
            keys.push(desc);
        }
    }

    Ok((keys[0].clone(), keys[1].clone()))
}
