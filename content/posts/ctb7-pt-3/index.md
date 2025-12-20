---
title: "capture the bitcoin at tab7: pt 3"
date: 2025-12-20
---

Posts in this series:

1. [capture the bitcoin at tab7: pt 1]({{< relref "ctb7-pt-1" >}})
1. [capture the bitcoin at tab7: pt 2]({{< relref "ctb7-pt-2" >}})
1. capture the bitcoin at tab7: pt 3

Previously, we solved the wizards' riddles and stole their bitcoin, used the coin to find the correct preimage, deduced the mnemonic puzzle, and helped the sad pleb prevent the Giggler from enriching himself. We now find the path blocked by [a gate with two locks](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate.html).

## gate analysis

Before us is a gate with two locks: lock A and lock B. We have the option to visit each lock. So let's [start with lock A](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate/alock.html).

### visiting lock A

Lock A is a happy little dude, more than willing to talk to us, especially since we just didn't walk around the gate like the Giggler did. He then treats us to a large language locks poem. Lastly, he lets us in on a little secret about his brother, lock B. He suggests we look at his `/bottom` to find his secrets. Let's keep that in mind while we visit lock B. Perhaps he can shed some light on the way forward.

### visiting lock B

We [navigate to lock B](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate/block.html) and find a very different kind of lock. This one is a grumpy douche who doesn't want to give us the time of day. However, he does tell us that his brother, lock A, used loaded dice to generate entropy for his seed phrase. This is a good clue. Let's go back to lock A and see if we can crack his seed phrase.

### solving lock A

Lock B told us his brother used loaded dice to generate entropy for his seed phrase. We can assume this clue is meant to tell us that his seed phrase is low entropy, meaning there are probably a bunch of repeated words in the mnemonic phrase. If we look closer at the LLL poem, we can see that there is one particular word that is repeated eleven times: abandon. We can also make an assumption that this little homey put his seed phrase in order into some AI and told it to make a poem. This means the final word is one of the words after the final "abandon" in the poem. I personally did not know the last word, but it seems this is a well-known low-entropy mnemonic phrase. The final word turns out to be "about."

I wrote a small program to output the address associated with this seed phrase. For anyone following along with this series, this code will be no surprise. Notice, I made an assumption that this was a phrase to an Xpriv, and, like the other challenges, the key pair we needed was at the standard Taproot derivation path: `m/86'/0'/0'/0/0`.

```rust
use anyhow::Result;
use bdk_wallet::bip39::Mnemonic;
use bdk_wallet::bitcoin::bip32::{DerivationPath, Xpriv};
use bdk_wallet::bitcoin::secp256k1::Secp256k1;
use bdk_wallet::bitcoin::{Address, KnownHrp, Network};

const LOCK_A_PHRASE: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

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

    Ok(())
}
```

The important part here is the address, which will be `bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr`. Looking at this address [in a block explorer](https://mempool.space/address/bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr) and exploring its history, we can see a transaction with [a suspicious `OP_RETURN` output](https://mempool.space/tx/f8a8c0656ed97d0c4181185b9f439f89aa42e6da223a4318c5f039a925cc0395). It says "concat A and B to continue. A = /becarefulth." Well that's helpful. So we are told we need two pieces of information, one from each lock, and we need to concatenate them to continue.

Now that we have what we need from lock A, let's unlock lock B.

### solving lock B

Lock A gave us a great clue on how to solve lock B. He told us we need to look at his `/bottom`. As invasive as this is, we need to stop the Giggler. So checking out his bottom is what we shall do. Appending `/bottom` to lock B's URL [gives us more information](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate/block/bottom). We are given what is obviously an Xpriv and another image of lock B with a curious set of numbers on what I can only assume would be his forehead. It looks an awful lot like a derivation path. Let's add to our previous program and try to derive another key pair using this new information.

```rust
use std::str::FromStr;

const LOCK_B_XPRIV: &str = "xprv9s21ZrQH143K37yRWQhUGQhQXuqoodSPVYhTsE9jmsJXmSu8GvhcXJvcJrNbLNk45BhFdmr2bKVKf8bFbhZ1cUma7WYCJCCoZQwENDgoh5F";

// ...

let derivation_path = DerivationPath::from_str("m/7'/7'/7'/7/7")?;
let lock_b_xpriv = Xpriv::from_str(LOCK_B_XPRIV)?;
let lock_b_keypair = lock_b_xpriv
     .derive_priv(&secp, &derivation_path)?
     .to_keypair(&secp);
let (lock_b_pubkey, _) = lock_b_keypair.x_only_public_key();
let lock_b_address = Address::p2tr(&secp, lock_b_pubkey, None, KnownHrp::Mainnet);

println!("Lock B address: {}", lock_b_address);
println!("Lock B public key: {}", lock_b_pubkey);
```

Again, we want the address, which will be `bc1palqa6u7yzwzrjltymt073pn9x93exqdq4hxh33u4xsfahdk9uz4st6msz5`. Viewing that [in our block explorer](https://mempool.space/address/bc1p92l58rumrwfhnsham6cluv703kx27l8z4cuj8ny6rqku7asy4p2sms0lgu), we see two transactions. At the time of the CTB, there [would have been only one](https://mempool.space/tx/affb6c4254034b6a686c3819ddf921e7a1b7070b3451b9efdcc4b39932e4b4f0). This transaction, like the previous one, has an `OP_RETURN` output with "egrassislava."

Now that we have both pieces, let's see if we can unlock the gate by appending `/becarefulthegrassislava` to [the gate page's URL](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate/becarefulthegrassislava).

## eye always feel like somebody's watching me

We make our way through the gate and find ourselves at a fork in the road with three directions to choose from. We are also given a descriptor to sweep the funds locked by the two keys we just received. Let's try and enrich ourselves before moving on.

To be able to simulate the sweep locally, we can use `nigiri` again.

```bash
nigiri start
nigiri faucet bcrt1pzxkk8vc9pu4mz36gra95rpptavxjlmretzkpx3s7u4gj37254nmscd4tk6 0.00249822
nigiri rpc getnewaddress '' bech32
```

We can add to the program we've been writing to create a transaction to sweep the funds like we have done multiple times at this point. Below is the code to add to make the transaction. Like usual, you'll need to update the previous outpoint and `to_address` to match your nigiri local environment.

```rust
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

const DESCRIPTOR: &str = "tr(ff13a3311d3e14239bcbb9dfd5d304f4b57c32c0b35d313cb5255f93d7b2dc68,multi_a(2,cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115,88c4bc76935bec829f0f46bf0f22436f5e20cac10116d7efbeaf0e8e05b0d8d6))#e8t3gujw";

// ...

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
```

Notice the order of the signatures in our witness. We push lock B's signature first since we need lock A's signature to be the first signature validated. Test that against the local mempool, and broadcast it if it's accepted.

```bash
# test the transaction is valid
nigiri rpc testmempoolaccept "[\"$(cargo run --bin twolocksonegate | tail -n 1)\"]"

# broadcast the transaction
nigiri push "$(cargo run --bin twolocksonegate | tail -n 1)"
```

## and there are many paths to tread

With three choices of paths to choose from, we decide to camp out here. Join us next time to find out what each path has in store for us.
