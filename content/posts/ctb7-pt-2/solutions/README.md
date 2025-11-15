```bash
nigiri start
nigiri faucet bcrt1prg82xfnv265zvl0mt6q2rr39mpmskrf2nyum4hqnmdwen3kzsn3slts4sh 0.00250000
nigiri rpc getnewaddress '' bech32
nigiri rpc testmempoolaccept "[\"$(cargo run -q --bin wizards)\"]"
```

```bash
nigiri start
nigiri faucet bcrt1puzn2h3v8n7kk29p580ewkeunza8n6ed58l6dh9tjtyhtj00ntncqzrqwx6 0.00052069
nigiri rpc getnewaddress '' bech32
nigiri rpc testmempoolaccept "[\"$(cargo run -q --bin crying)\"]"
```

