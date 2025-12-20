use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use bdk_wallet::bip39::Mnemonic;
use bdk_wallet::bitcoin::absolute::LockTime;
use bdk_wallet::bitcoin::bip32::{DerivationPath, Xpriv};
use bdk_wallet::bitcoin::consensus::Encodable;
use bdk_wallet::bitcoin::hex::prelude::*;
use bdk_wallet::bitcoin::secp256k1::{Message, Secp256k1};
use bdk_wallet::bitcoin::sighash::{Prevouts, SighashCache};
use bdk_wallet::bitcoin::transaction::Version;
use bdk_wallet::bitcoin::{
    Address, Amount, KnownHrp, Network, OutPoint, ScriptBuf, TapSighashType, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};
use miniscript::Descriptor;

const LOCK_A_PHRASE: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const LOCK_B_XPRIV: &str = "xprv9s21ZrQH143K37yRWQhUGQhQXuqoodSPVYhTsE9jmsJXmSu8GvhcXJvcJrNbLNk45BhFdmr2bKVKf8bFbhZ1cUma7WYCJCCoZQwENDgoh5F";
const DESCRIPTOR: &str = "tr(ff13a3311d3e14239bcbb9dfd5d304f4b57c32c0b35d313cb5255f93d7b2dc68,multi_a(2,cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115,88c4bc76935bec829f0f46bf0f22436f5e20cac10116d7efbeaf0e8e05b0d8d6))#e8t3gujw";

fn main() -> Result<()> {
    let secp = Secp256k1::new();

    let derivation_path = DerivationPath::from_str("m/86'/0'/0'/0/0")?;
    let phrase = Mnemonic::from_str(LOCK_A_PHRASE)?;
    let lock_a_xpriv = Xpriv::new_master(Network::Bitcoin, &phrase.to_seed(""))?;
    let lock_a_keypair = lock_a_xpriv
        .derive_priv(&secp, &derivation_path)?
        .to_keypair(&secp);
    let (lock_a_pubkey, _) = lock_a_keypair.x_only_public_key();
    let lock_a_address = Address::p2tr(&secp, lock_a_pubkey, None, KnownHrp::Mainnet);

    println!("Lock A address: {}", lock_a_address);
    println!("Lock A public key: {}", lock_a_pubkey);

    let derivation_path = DerivationPath::from_str("m/7'/7'/7'/7/7")?;
    let lock_b_xpriv = Xpriv::from_str(LOCK_B_XPRIV)?;
    let lock_b_keypair = lock_b_xpriv
        .derive_priv(&secp, &derivation_path)?
        .to_keypair(&secp);
    let (lock_b_pubkey, _) = lock_b_keypair.x_only_public_key();
    let lock_b_address = Address::p2tr(&secp, lock_b_pubkey, None, KnownHrp::Mainnet);

    println!("Lock B address: {}", lock_b_address);
    println!("Lock B public key: {}", lock_b_pubkey);

    let Descriptor::Tr(taproot) = Descriptor::<XOnlyPublicKey>::from_str(DESCRIPTOR)? else {
        bail!("not a taproot descriptor");
    };
    let previous_address = taproot.address(Network::Regtest);
    assert_eq!(
        previous_address.to_string(),
        "bcrt1pzxkk8vc9pu4mz36gra95rpptavxjlmretzkpx3s7u4gj37254nmscd4tk6"
    );

    let total = Amount::from_str("0.00249822 BTC")?;
    let previous_output = TxOut {
        value: total,
        script_pubkey: previous_address.script_pubkey(),
    };

    let previous_outpoint =
        OutPoint::from_str("57d6716ca1795c58eba8e349dc309eeff6b0d0c60921ce826a72ae979b5896fd:1")?;
    let txin = TxIn {
        previous_output: previous_outpoint,
        ..Default::default()
    };

    let to_address = Address::from_str("bcrt1qk2kdgxe95qvyvjj5nud8x6kurxz29ztq6wpqv8")?
        .require_network(Network::Regtest)?;
    let txout = TxOut {
        value: Amount::ZERO,
        script_pubkey: to_address.script_pubkey(),
    };
    let opreturn = TxOut {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::new_op_return(b"busted the locks"),
    };

    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, opreturn],
    };
    let fee = Amount::from_sat(tx.vsize() as u64 * 50);
    tx.output[0].value = total - fee;

    let spend_info = taproot.spend_info();
    let leaf = spend_info
        .leaves()
        .next()
        .ok_or(anyhow!("taptree leaf missing"))?;

    let mut sighash_cache = SighashCache::new(&tx);
    let prevouts = Prevouts::All(&[previous_output]);
    let sighash = sighash_cache.taproot_script_spend_signature_hash(
        0,
        &prevouts,
        leaf.leaf_hash(),
        TapSighashType::Default,
    )?;
    let message = Message::from_digest(*sighash.as_ref());
    let lock_a_signature = secp.sign_schnorr(&message, &lock_a_keypair);
    let lock_b_signature = secp.sign_schnorr(&message, &lock_b_keypair);

    let mut witness = Witness::new();
    witness.push(lock_b_signature.serialize());
    witness.push(lock_a_signature.serialize());
    witness.push(leaf.script());
    witness.push(leaf.into_control_block().serialize());
    tx.input[0].witness = witness;

    let mut bytes = Vec::new();
    tx.consensus_encode(&mut bytes)?;
    println!("{}", bytes.to_lower_hex_string());

    Ok(())
}
