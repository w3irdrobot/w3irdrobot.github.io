use std::str::FromStr;

use anyhow::{Result, anyhow, bail};
use bitcoin::absolute::LockTime;
use bitcoin::consensus::Encodable;
use bitcoin::hashes::{Hash, sha256};
use bitcoin::hex::prelude::*;
use bitcoin::key::Keypair;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::transaction::Version;
use bitcoin::{
    Address, Amount, Network, OutPoint, ScriptBuf, TapSighashType, Transaction, TxIn, TxOut,
    Witness, XOnlyPublicKey,
};
use miniscript::Descriptor;

// START constants
const DESCRIPTOR: &str = "tr(ff13a3311d3e14239bcbb9dfd5d304f4b57c32c0b35d313cb5255f93d7b2dc68,and_v(v:pk(b064bd37b71b7c39355f866e022287097757b05bb1790ecffc5de5fbf07cff69),sha256(a02e93b3dce9cb6031055905d94b04516819ef8fdae4c498777350469e6352dd)))#pygtzret";
const PREIMAGE: &str = "366d2eff102fd290e7be0226f3dd746f4657bce85f08eeef55d0a7777d1a313f";
const PRIVATE_KEY: &str = "0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae";
// END constants

fn main() -> Result<()> {
    // START setup
    let Descriptor::Tr(taproot) = Descriptor::<XOnlyPublicKey>::from_str(DESCRIPTOR)? else {
        bail!("not a taproot descriptor");
    };
    let previous_address = taproot.address(Network::Regtest);
    assert_eq!(
        previous_address.to_string(),
        "bcrt1prg82xfnv265zvl0mt6q2rr39mpmskrf2nyum4hqnmdwen3kzsn3slts4sh"
    );

    let total = Amount::from_str("0.0025 BTC")?;
    let previous_output = TxOut {
        value: total,
        script_pubkey: previous_address.script_pubkey(),
    };

    let previous_outpoint =
        OutPoint::from_str("329882a72aa66c446806109f214ba8ffdd0f62300f4a4607c82f34762f898e3b:0")?;
    let txin = TxIn {
        previous_output: previous_outpoint,
        ..Default::default()
    };

    let to_address = Address::from_str("bcrt1qsrk86mvplp46qvrt80zxwc07g3mdzfnwuyp74x")?
        .require_network(Network::Regtest)?;
    let txout = TxOut {
        value: Amount::ZERO,
        script_pubkey: to_address.script_pubkey(),
    };
    let opreturn = TxOut {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::new_op_return(b"stole the wizard's treasure"),
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
            == "a02e93b3dce9cb6031055905d94b04516819ef8fdae4c498777350469e6352dd"
    );

    let secp = Secp256k1::new();
    let keypair = Keypair::from_seckey_str(&secp, PRIVATE_KEY)?;
    let public_key = XOnlyPublicKey::from(keypair.public_key());
    assert_eq!(
        public_key.to_string(),
        "b064bd37b71b7c39355f866e022287097757b05bb1790ecffc5de5fbf07cff69"
    );
    // END load

    // START sign
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
    // END sign

    // START witness
    let mut witness = Witness::new();
    witness.push(preimage);
    witness.push(signature.serialize());
    witness.push(leaf.script());
    witness.push(leaf.into_control_block().serialize());
    tx.input[0].witness = witness;
    // END witness

    // START output
    let mut bytes = Vec::new();
    tx.consensus_encode(&mut bytes)?;
    println!("{}", bytes.to_lower_hex_string());
    // END output

    Ok(())
}
