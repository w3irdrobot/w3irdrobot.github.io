---
title: "capture the bitcoin at tab7: pt 2"
date: 2025-12-07
lastmod: 2025-12-19
---

Posts in this series:

1. [capture the bitcoin at tab7: pt 1]({{< relref "ctb7-pt-1" >}})
1. capture the bitcoin at tab7: pt 2

Last week, [we started our journey]({{< relref "ctb7-pt-1" >}}) toward solving the TAB7 CTB. We [ended in a wizard's lab](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab.html), with some trippy looking wizards playing scientist and a strange door looming behind them.

## behind door number one

[Going through the door](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/ilikebigscriptsandicannotlie.html) puts us in a room, eavesdropping on a conversation between two wizards. They are discussing how to lock up some funds. We are given a descriptor with a script path with two requirements: a signature and a preimage. From the conversation, we know the private key to sign with is the TXID of "the big 4 megger" transaction. A quick search turns up this transaction with TXID [0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae](https://mempool.space/tx/0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae). This matches the hint we are given that it starts with `03`. To get the preimage, we are supposed to "use the hash of the transaction." So we need to take the hex contents of that transaction and run it through SHA256 to get our preimage. We can do this from the terminal like so:

```bash
₿ curl -s https://mempool.space/api/tx/0301e0480b374b32851a9462db29dc19fe830a7f7d7a88b81612b9d42099c0ae/hex | \
    xxd -r -p | \
    sha256sum
366d2eff102fd290e7be0226f3dd746f4657bce85f08eeef55d0a7777d1a313f  -
```

Now that we have the data we need to unlock the funds, we need to construct a transaction to send the wallets somewhere we control. I used Rust and the `bitcoin` crate to do this. Since the CTB is over and the funds have already been taken, I needed a way to test that my program created a valid transaction. Therefore, I used [nigiri](https://nigiri.vulpem.com/) to setup a local regtest environment to test against. The outputs used will be different than mainnet, but the idea works the same. To set things up similar to how they are on mainnet, you can run the following:

```bash
# Start the local regtest environment
nigiri start
# Fund the address we will be sweeping funds from (we'll discuss this address later)
nigiri faucet bcrt1prg82xfnv265zvl0mt6q2rr39mpmskrf2nyum4hqnmdwen3kzsn3slts4sh 0.00250000
# Get an address we control to sweep funds to
nigiri rpc getnewaddress '' bech32
```

Setup a new Rust project and install the dependencies:

```shell
cargo new solutions
cd solutions
cargo add anyhow miniscript
cargo add bitcoin -F rand-std
```

In the program file, add a few constants for each piece of data we need to unlock the funds.

{{< code language="rust" source="solutions/src/wizards.rs" id="constants" >}}

Inside the main function, we need to parse the descriptor, setup the previous outpoint, create our outputs, and create the transaction using the values we received from the earlier `nigiri` commands. The `previous_address` was found by running this program with just those lines and seeing what was output. Since we know this descriptor is correct, we can rely on that. The amount comes from the output that was already spent on mainnet. The index of the output we want to spend was obtained by using the Esplora instance nigiri sets up for us at `localhost:5000` to look up the created transaction. We added an `OP_RETURN` just for fun; it's not a necessity. Lastly, we update the amount of the main output to account for the fee. This isn't super accurate, but it's close enough for our purposes.

{{< code language="rust" source="solutions/src/wizards.rs" id="setup" >}}

Next, we load in our preimage and private key and make some assertions to make sure we are using the correct values by comparing them to what is in the descriptor.

{{< code language="rust" source="solutions/src/wizards.rs" id="load" >}}

Next, comes the fun part: taproot script spending. First, we generate the sighash for our spend path and sign it with our private key.

{{< code language="rust" source="solutions/src/wizards.rs" id="sign" >}}

Construct the witness and add it to the transaction's input. We push the preimage on first and then the signature because the signature needs to be on top of the stack when the rest of the Script is evaluated. The witness structure for a Taproot script spend is defined in [BIP-341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#script-validation-rules). The [Learn Me A Bitcoin](https://learnmeabitcoin.com/technical/upgrades/taproot/#script-path-spend) article on Taproot also has a great breakdown.

{{< code language="rust" source="solutions/src/wizards.rs" id="witness" >}}

Lastly, we encode our transaction in hex and print it to the terminal.

{{< code language="rust" source="solutions/src/wizards.rs" id="output" >}}

It's possible much of this setup and can be automated by using `bdk_wallet` and syncing to an Esplora instance. I couldn't figure it out, but if someone knows how to do this, please [reach out](https://njump.me/npub17q5n2z8naw0xl6vu9lvt560lg33pdpe29k0k09umlfxm3vc4tqrq466f2y).

We can submit this to nigiri to see if it works.

```shell
nigiri rpc testmempoolaccept "[\"$(cargo run -q --bin wizards)\"]"
```

**Note: I have [multiple binaries](https://github.com/w3irdrobot/w3irdrobot.github.io/blob/master/content/posts/ctb7-pt-2/solutions/Cargo.toml) setup in my `Cargo.toml` file. Depending on how you setup your project, you may not need the `--bin wizards` flag.**

Assuming we did everything correctly, this should come back with information about our transaction and no errors. If so, then we can broadcast it to our local regtest environment.

```shell
nigiri push "$(cargo run -q --bin wizards)"
```

Check the address we swept funds to in the local Esplora to make sure it worked. Now let's head back to the lab and see what else the wizards have in store for us.

## back in the lab again

Back in the lab, the wizards are cooking up some hashes. We get a clue the coin has something to do with it. We gotta make some hashes and try their first three bytes in the URL to find the next page. Also, there's a hint telling us the delimiter to use.

If you remember in part one, our [coin had some words](../ctb7-pt-1/#the-starting-line) on one side: "PERMISSIONLESS INNOVATION". We can assume we are using LF line-endings. One thing to know from experience, this line-ending is only between the words, as it is on the coin, not at the end of the sentence. Because of how hashes work, one extra LF will give us the wrong answer. Here is a one-liner to get the correct hash:

```bash
₿ printf 'PERMISSIONLESS\nINNOVATION' | sha256sum | head -c 6
b64728
```

This gives us `b64728`. Removing `.html` from the URL and adding our answer sends us to [the next page](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728)!

## why so mnemonic?

The next page just says we need to solve the puzzle and get the first three bytes of some transaction ID. The puzzle is a set of twelve images, three rows of four. We've been given challenges like this in past years. We need to figure out the seed phrase to a wallet that will most likely have funds in it. It sounds like we need to get the transaction ID of the first transaction of the wallet to move forward.

This is usually a game of "guess and test." My preferred way to solve this is to use a tool that can mass-check seed phrases, especially if you can queue up multiple words for the same place in the phrase. [A teammmate](https://github.com/ildarmgt) has put together [a tool for doing just that](https://codepen.io/ildarmgt/full/vYbJQmp). It turns out the answer is the following:

```
code clown giggle when return filter appear useless then update core baby
```

There is [one address transaction](https://mempool.space/address/bc1pdfhnlf7jtzq04rwm53m47ezdrq5he94hkdyk387278wlxzdc5jvsyuus0m) connected to it. Here is [its funding transaction](https://mempool.space/tx/d49fa5aeb0557c4c377ee582d1784aa96ad0b356522ac9f71d11bbb041688234?mode=details). Let's try the first three bytes in the URL and [see what happens](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5).

## cry harder

Looks like that works! We come across a pleb whose funds have been taken. That darn Giggler is at it again! It looks like this pleb wants us to secure his funds so the Giggler can't steal the rest. Using the Xpriv and descriptor, we can make a program similar to our previous one to sweep the funds to an address we control.

First, we can assume, since we have been dealing with Taproot descriptors so far, that the private key is the first key derived from the given Xpriv using the standard Taproot derivation path: `m/86'/0'/0'/0/0`. Next, by looking at the descriptor, we know we are looking for another preimage. The clue we are given is that the Giggler has already stolen one of the outputs. This means that the preimage has already been revealed on-chain. If we look at [the funding transaction](https://mempool.space/tx/56475ce4887d797c222c1fb1a068cb03a995166ac2940552f42a3ac9dae79170) we were given at the beginning of our quest, we can see there [is one address](https://mempool.space/address/bc1puzn2h3v8n7kk29p580ewkeunza8n6ed58l6dh9tjtyhtj00ntncqcju8f0) that had two outputs in it. If we had seen this at the time of the CTB, we would have seen only one was spent. If we dig into [that transaction](https://mempool.space/tx/798fc978df568a7ebccb8edbaeaf2a5d2f95bf0212aae50af271d5ae485702d9), and look at the witness data for the input, we can see that the first item is the preimage we are looking for: `33f139bd7f306d6e50644fb8fa76072e5654ced0839f51605feb7cd9106f754a`.

We can reuse the environment from our first program to test our solution.

```shell
nigiri start
nigiri faucet bcrt1puzn2h3v8n7kk29p580ewkeunza8n6ed58l6dh9tjtyhtj00ntncqzrqwx6 0.00052069
nigiri rpc getnewaddress '' bech32
```

In a new file, we can add some constants like before.

{{< code language="rust" source="solutions/src/crying.rs" id="constants" >}}

Let's setup our transaction. This should all look very familiar.

{{< code language="rust" source="solutions/src/crying.rs" id="setup" >}}

Next, we load in our preimage and private key. Getting the private key is slightly different this time around. Since we have an Xpriv instead of the private key, we need to derive the private key from the Xpriv.

{{< code language="rust" source="solutions/src/crying.rs" id="load" >}}

Lastly, we sign the transaction, construct the witness, add it to the transaction, and output the encoded transaction to the terminal.

{{< code language="rust" source="solutions/src/crying.rs" id="rest" >}}

Once again, testing this should work.

```shell
nigiri rpc testmempoolaccept "[\"$(cargo run -q --bin crying)\"]"
```

We can finally sweep the funds by submitting it to our node.

```shell
nigiri push "$(cargo run -q --bin crying)"
```

Now that the funds are safe, [we can move on](https://tabctb.com/0x07/wristybusiness/lore/7AB7/lab/b64728/d49fa5/abgate.html).

## locks on lock on locks

Arriving at a gate with two locks, we decide to take a rest. Come back next time to find out how to get through the gate and its quirky locks.
