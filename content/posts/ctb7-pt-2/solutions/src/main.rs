// large tx: https://mempool.space/tx/0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae
// sha256 digest of the tx hex: 302a8df9fb299d57d33bc728b6c761b6a8657a3dbdfc67abdd623a33d0b4c726

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
    Address, Amount, Network, OutPoint, TapSighashType, Transaction, TxIn, TxOut, Witness,
    XOnlyPublicKey,
};
use miniscript::Descriptor;

const DESCRIPTOR: &str = "tr(ff13a3311d3e14239bcbb9dfd5d304f4b57c32c0b35d313cb5255f93d7b2dc68,and_v(v:pk(b064bd37b71b7c39355f866e022287097757b05bb1790ecffc5de5fbf07cff69),sha256(a02e93b3dce9cb6031055905d94b04516819ef8fdae4c498777350469e6352dd)))#pygtzret";
const PREIMAGE: &str = "366d2eff102fd290e7be0226f3dd746f4657bce85f08eeef55d0a7777d1a313f";
const PRIVATE_KEY: &str = "0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae";

fn main() -> Result<()> {
    let Descriptor::Tr(taproot) = Descriptor::<XOnlyPublicKey>::from_str(DESCRIPTOR)? else {
        bail!("not a taproot descriptor");
    };
    assert_eq!(
        taproot.address(Network::Regtest).to_string(),
        "bcrt1prg82xfnv265zvl0mt6q2rr39mpmskrf2nyum4hqnmdwen3kzsn3slts4sh"
    );
    let total = Amount::from_str("0.0025 BTC")?;
    let previous_output =
        OutPoint::from_str("44afabb1b631d2cd265910dc6befef7e3962df486ad4a4a12f83668c58c9c22c:0")?;
    let txin = TxIn {
        previous_output,
        ..Default::default()
    };

    let to_address = Address::from_str("bcrt1q9th3z6mknxp60avl9xq8vac05q4xgvygx7r7jz")?
        .require_network(Network::Regtest)?;
    let txout = TxOut {
        value: Amount::ZERO,
        script_pubkey: to_address.script_pubkey(),
    };

    let outputs = vec![txout];
    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: outputs.clone(),
    };
    // Basic 5 sats/vbyte fee
    let fee = Amount::from_sat(tx.vsize() as u64 * 5);
    tx.output[0].value = total - fee;

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

    let spend_info = taproot.spend_info();
    let leaf = spend_info
        .leaves()
        .next()
        .ok_or(anyhow!("taptree leaf missing"))?;

    let mut sighash_cache = SighashCache::new(&tx);
    let prevouts = Prevouts::All(&outputs);
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
    witness.push(signature.as_ref());
    witness.push(leaf.script());
    witness.push(leaf.into_control_block().serialize());
    tx.input[0].witness = witness;

    let mut bytes = Vec::new();
    tx.consensus_encode(&mut bytes)?;
    println!("{}", bytes.to_lower_hex_string());

    Ok(())
}
