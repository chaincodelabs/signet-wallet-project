# Week 1: Recover Wallet State

Challenge: given a descriptor and a blockchain, compute your confirmed wallet
balance. Submit a program with your `xprv` hard-coded, execute the necessary
bitcoin-cli RPCs and return your wallet balance as float with 8 decimal places.

We will run your program against Bitcoin Core synced to our private signet chain
but with the Bitcoin Core wallet *disabled*. That means that RPCs like
`importdescriptor` will fail. You are, of course, allowed to import your
descriptor into Bitcoin Core to check your own work as you develop the wallet locally.

## Steps

1. Decode the base58 `xprv` and extract the private key and chaincode
2. Derive the key and chaincode at the path in the descriptor (`84h/1h/0h/0`)
3. Derive 2000 private keys from that path
4. Compute the compressed public key for each private key
5. Compute the `p2wpkh` witness program for each compressed public key
6. Using the RPC interface **of your own local, synced signet node**, scan all transactions in the first 310 blocks in the chain
    1. Look for your witness programs in all TX outputs - these are coins you received
    2. Look for you compressed public keys in all TX witnesses - these are coins you spent
    3. You will need to track received coins by their **outpoint** so you can deduct their value from your balance when they are spent
7. Keep a running total of your wallet balance and return its name and final value

## Submission

Each student will get a private fork of this repository when they accept the
GitHub Classroom assignment. You will commit and push your submissions to GitHub
which will evaluate the answers automatically. You can commit and push as often
as you like and GitHub will re-evaluate your code every time.

Your code will be executed in an environment with a synced signet full node,
so any `bitcoin-cli -signet ...` commands executed in the shell should work
just like they do for you locally.

Your code must return exactly one line, a string, with your wallet name followed
by a single space and your wallet balance in tBTC with 8 decimal places
(see the [example](#example-output) below).

You are allowed to write your wallet code in any of the following programming
languages:

- Python
- C
- C++
- C#
- Rust
- Go

The autograder runs in Ubuntu 22 with
[these packages](https://github.com/actions/runner-images/blob/ubuntu22/20231217.2/images/ubuntu/Ubuntu2204-Readme.md)
installed by GitHub.

The autograder will run the bash script [solution/run_balance.sh](solution/run_balance.sh) which
MAY BE EDITED BY STUDENTS if you need to install additional dependencies. Only
the very last line of that script's output will be evaluated so make sure your
code runs last in the script, and prints the answer last!

The default language for this exercise and the autograder is Python. The easiest
way to complete this project is to complete the obfuscated code template in
[solution/python/balance.py](solution/python/balance.py). No other files need
to be modified unless you want to start from scratch in Python or write in one
of the other languages.

There is also a Rust template in [solution/rust/balance/src](solution/rust/balance/src).
If you choose to work in Rust you will need to modify [solution/run_balance.sh](solution/run_balance.sh)
to execute the Rust code.

You **MAY** import an ECDSA library to access constants like `G` or the order
of the curve, but you **MAY NOT** use a Bitcoin-specific library to avoid implementing
BIP32 yourself.

If you choose to write code in something other than Python you MUST modify
[solution/run_balance.sh](solution/run_balance.sh) as well so that your code is compiled and
executed appropriately!

## Hints

- [BIP32: Hierarchical Deterministic Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP32 test vectors](https://en.bitcoin.it/wiki/BIP_0032_TestVectors)
- Be careful with floating-point precision! You may want to convert to integers (satoshis)
    - If you are using Python you may want to specify `json.loads(...,  parse_float=Decimal)`
- If a BIP32 index is "hardened", you must add the offset `0x80000000` to the index before deriving the key
        - e.g. index `1h` -> `0x80000001`
- Look for your own compressed public keys as the second item (`items[1]`) in each witness stack
- Learn how to use certain RPC commands with (for example) `bitcoin-cli -signet help getblock`

## Example output

```sh
# My wallet descriptor is
# wpkh(tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ/84h/1h/0h/0/*)#nayduu7d
$ python balance.py
wallet_000 1644.12055731
```
