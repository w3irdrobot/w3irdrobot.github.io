use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use bitcoin::absolute::LockTime;
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::consensus::Encodable;
use bitcoin::hashes::{sha256, Hash};
use bitcoin::hex::prelude::*;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::transaction::Version;
use bitcoin::{
    Address, Amount, Network, OutPoint, ScriptBuf, TapSighashType, Transaction, TxIn, TxOut,
    Witness, XOnlyPublicKey,
};
use miniscript::Descriptor;

// START constants
const DESCRIPTOR: &str = "tr(ff13a3311d3e14239bcbb9dfd5d304f4b57c32c0b35d313cb5255f93d7b2dc68,and_v(v:pk(22313a9138804d91e988f0c8c68144491498057fd81150519b4b85cbe60b45b9),sha256(2c00c2867ba2bb97b5a43804a28f7f498b37d4c43969ad3dbb0b0d0763ba7ff5)))#gxh3fatq";
const PREIMAGE: &str = "33f139bd7f306d6e50644fb8fa76072e5654ced0839f51605feb7cd9106f754a";
const XPRIV: &str = "xprv9s21ZrQH143K4bSLr58a4KVW8rJVZhXkDHXLZsxiokqN1AMJr2pEmqNLSbgDPDcSRDp6EWQriXPQJwDXFZK4R3sZRh59G91sjoK55VM2MrU";
// END constants

fn main() -> Result<()> {
    // START setup
    let Descriptor::Tr(taproot) = Descriptor::<XOnlyPublicKey>::from_str(DESCRIPTOR)? else {
        bail!("not a taproot descriptor");
    };
    let previous_address = taproot.address(Network::Regtest);
    assert_eq!(
        previous_address.to_string(),
        "bcrt1puzn2h3v8n7kk29p580ewkeunza8n6ed58l6dh9tjtyhtj00ntncqzrqwx6"
    );

    let total = Amount::from_str("0.00052069 BTC")?;
    let previous_output = TxOut {
        value: total,
        script_pubkey: previous_address.script_pubkey(),
    };

    let previous_outpoint =
        OutPoint::from_str("6c6d0aa38e90e24e0ef74a832a5b8dd5bb0e9d69af8a33f1316033648a838bad:0")?;
    let txin = TxIn {
        previous_output: previous_outpoint,
        ..Default::default()
    };

    let to_address = Address::from_str("bcrt1qrevdmwd6lnyhefkk3ctl5r9y2r6lsl8hd76xkt")?
        .require_network(Network::Regtest)?;
    let txout = TxOut {
        value: Amount::ZERO,
        script_pubkey: to_address.script_pubkey(),
    };
    let opreturn = TxOut {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::new_op_return(b"cry harder"),
    };

    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, opreturn],
    };
    let fee = Amount::from_sat(tx.vsize() as u64 * 50);
    tx.output[0].value = total - fee;
    // END setup

    // START load
    let preimage: [u8; 32] = FromHex::from_hex(PREIMAGE)?;
    let preimage_hash = sha256::Hash::hash(&preimage);
    assert!(
        preimage_hash.to_string()
            == "2c00c2867ba2bb97b5a43804a28f7f498b37d4c43969ad3dbb0b0d0763ba7ff5"
    );

    let secp = Secp256k1::new();
    let xpriv = Xpriv::from_str(XPRIV)?;
    let path = DerivationPath::from_str("m/86'/0'/0'/0/0")?;
    let pvt_key = xpriv.derive_priv(&secp, &path)?;
    let keypair = pvt_key.to_keypair(&secp);
    let public_key = XOnlyPublicKey::from(keypair.public_key());
    assert_eq!(
        public_key.to_string(),
        "22313a9138804d91e988f0c8c68144491498057fd81150519b4b85cbe60b45b9"
    );
    // END load

    // START rest
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
    let signature = secp.sign_schnorr(&message, &keypair);

    let mut witness = Witness::new();
    witness.push(preimage);
    witness.push(signature.serialize());
    witness.push(leaf.script());
    witness.push(leaf.into_control_block().serialize());
    tx.input[0].witness = witness;

    let mut bytes = Vec::new();
    tx.consensus_encode(&mut bytes)?;
    println!("{}", bytes.to_lower_hex_string());
    // END rest

    Ok(())
}
