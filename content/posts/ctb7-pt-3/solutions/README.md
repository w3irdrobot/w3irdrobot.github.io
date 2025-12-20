```bash
nigiri start
nigiri faucet bcrt1pzxkk8vc9pu4mz36gra95rpptavxjlmretzkpx3s7u4gj37254nmscd4tk6 0.00249822
nigiri rpc getnewaddress '' bech32
nigiri rpc testmempoolaccept "[\"$(cargo run --bin twolocksonegate | tail -n 1)\"]"
```
