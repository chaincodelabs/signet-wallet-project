# Week 2: Send A Multisig Transaction

Challenge: extend your wallet program to spend your own coins. You will create
two transactions: one that spends from a single-key `p2wpkh` and funds a `p2wsh`
multisig, and a second transaction that spends from that multisig. You will
sign and broadcast both transactions to the signet network, and submit your code
that gnerates both transactions as hex-encoded strings.

You may copy-and-paste as much code from last week as needed, or package multiple
source code files together as long as the program works.

We will evaluate your code submission against the transactions in the signet chain
and again, use of the Bitcoin Core wallet is not allowed by your submitted code.

## Steps

1. Re-run last week's code to recover wallet state: 2000 key pairs, and all unspent coins
2. Create a 2-of-2 multisig script from the first two keys (indexes 0 & 1)
3. Compute the [`p2wsh` witness program](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WSH) from that script
4. Construct the first transaction (spend from p2wpkh):
    1. Choose one of your unspent coins for the input
    2. Add an output: 0.01 BTC output to your multisig witness program
    3. Another output: the change (minus fee!) back to your 0th key's public key hash witness program
    4. Compute the `SIGHASH_ALL` transaction digest for your input as specified in [BIP 143](https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#user-content-Specification)
    5. Sign the digest using the key responsible for the coin you are spending
    6. Create a transaction witness with the signature and public key
    7. Serialize the transaction (without witness data!) and compute the txid
    8. Serialize the final complete transaction
    9. Return both the txid (required to spend from the p2wsh) and the complete hex-encoded transaction.
5. Construct the second transaction (spend from p2wsh):
    1. Repeat the previous steps but spend the `p2wsh` multisig output you created in the last transaction as the input to the new transaction
    2. Send 0 BTC to an `OP_RETURN` output script which encodes your full name (or nym) in ASCII
    3. Don't forget the change output and fee! You can reuse your 0th key like before.
    4. Serialize the final transaction and return the hex encoded string.

## Show off

While not mandatory, you are encouraged to broadcast your transactions to the
signet network! You can use `bitcoin-cli -signet sendrawtransaction <hex>` for
this. It will be very cool to see everyone's name in our private signet blockchain!

## Submission

This assignment is a continuation of [Recover Balance](./recover_balance.md)
which you must have completed already to continue. You can push more commits to the same
repo to pass the second autograder test, which is executed by [solution/run_spend.sh](solution/run_spend.sh).
So, like last week, if you need to install additional packages for your project
you can modify that script.

You code must return exactly TWO lines, each line containing the raw hex string
for a valid transaction. (see the [example](#example-output) below).

The autograder will again run in an environment with a signet node that is stuck
on block 400. We do this so that even if you have already spent your coins and submitted
your transactions to the signet network, the autograder node will NOT process those
transactions and they can be evaluated again locally! Both transactions must be
accepted to the autograder node's mempool to pass.

Like last week, the default language is Python and an obfuscated code template
is yours to play with in [solution/python/spend.py](solution/python/spend.py).
If you choose to write in a different language you MUST edit [solution/run_spend.sh](solution/run_spend.sh).

There is also a Rust template in [solution/rust/spend/src](solution/rust/spend/src).
If you choose to work in Rust you will need to modify [solution/run_spend.sh](solution/run_spend.sh)
to execute the Rust code.

You **MAY** import an ECDSA library to access constants like `G` or the order
of the curve, and you **MAY** use an external library for message signing (although
we encourage you to implement ECDSA signing yourself!). You **MAY NOT** use a
Bitcoin-specific library to avoid implementing BIP32 or structuring transaction
objects yourself.

## Hints

- [BIP 141: Segregated Witness (Consensus layer)](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki)
- [BIP 143: Transaction Signature Verification for Version 0 Witness Program](https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki)
- [Bitcoin protocol TX serialization](https://en.bitcoin.it/wiki/Protocol_documentation#tx)
- [Bitcoin script opcodes and their hex representation in raw scripts](https://en.bitcoin.it/wiki/Script)
- When constructing a multisig script use `OP_2` (not `0x02`) to indicate the number of required signatures and keys
- Other data like public keys must be pushed to the stack with length bytes (e.g. `0x21` for a 33-byte compressed public key)
- p2wsh witness programs do not commit to a length byte, just the raw script
- p2wsh witness programs are a *single* sha-256 hash of the raw script
- p2wpkh witness programs are the ripemd-160 hash of a sha-256 hash of the raw compressed public key
- For our purposes, all input sequences are `0xffffffff` and all transaction locktimes are `0x00000000`
- Since we are only concerned with segregated witness transactions, input `scriptSig` will always be empty (`0x00`, a 0-length script)
- The `scriptcode` in the transaction commitment must be prefixed with a length byte, but the witness program only commits to the raw script with no length byte
- The `SIGHASH_ALL` flag is one byte `0x01` when appended to a signature in a witness, but is four bytes little-endian `\x01\x00\x00\x00` in the transaction commitment
- Bitcoin signatures:
    - Must be strict-DER encoded
    - Must have the SIGHASH_ALL byte (`0x01`) appended
    - Must have a low s value as defined by [BIP 62](https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki#user-content-Low_S_values_in_signatures)
- Remember to add a `0x00` byte as the first witness stack element for the [CHECKMULTISIG bug](https://github.com/bitcoin/bips/blob/master/bip-0147.mediawiki)

See the obfuscated [example solution](example_solution/spend.py) which may be used as a template
for your project in Python, or as an example architecture for whatever language you choose.


## Example output

```sh
# My wallet descriptor is
# wallet_000: wpkh(tprv8ZgxMBicQKsPdCdx6Ypx5naxxjVfK43yefjvdoKU8rj8B3BaXRo8mXNteEXuVk34wprtzyLyq3gLt3vZPrcotU8Lv1UvK22AuVNdPPnLPu7/84h/1h/0h/0/*)#274t4g48
$ python spend_solution.py
02000000000101d62234a2ce378e85baad2d0499c60dbc3037b29da2cf4d4e796e52a6fd264d570000000000ffffffff0200e1f50500000000220020421305d3c9f5faee4f8fa7dc79f54c5eb2971ff4819cfac68fcb219c14f8aed5fb8d435500000000160014f1a65dab1d878c0ae057a5d3cc1b7d31f494d774024830450221008e9582cf252ed3b9f05c38a017c675ae8c55f0aba3bc9c1dfddb0fbbe3b17c3f022002d4c28e687597aa89445a78afdee34d96852493c140b573fa8b157edc3a704901210250e42d3f14ed353ffb49c8a041bab867dadbbb987a41007c7424aa7c5cad90ca00000000
020000000001015a2cc5b8b3023089035104bfb117cb78096011d7f0b1d22cc4242460d1af67cd0000000000ffffffff020000000000000000106a0e4d617474686577205a69706b696e18ddf50500000000160014f1a65dab1d878c0ae057a5d3cc1b7d31f494d77404004730440220311e223e3582608d45d1563774ebafe08f9c31c5ade6b1e553a2cfe803b59ab3022012929a2af9a56e8cbc13cf5df4eb5b2029ec2434835d49661d53b8ff183c4e06014730440220137d76b3701ecdbe45434cad77aecca251352141d5023d3fbe8b7ba33249852a022033ad7ffa4426b8e952a81aa097a9fd4610095cb6f801b3c5f61349ec7557db4001475221034837d61d3fef06d5eef3cd6fb1d52475afcfc933df442d15ead33bf963994ebb210277a243e2dd32d5425e160a7e5f5765e659556ff11f252fe4a10185038d3ed51e52ae00000000
```

The transactions generated in this example can be decoded by RPC:

### Spend from p2wpkh

```sh
$ bitcoin-cli decoderawtransaction 02000000000101d62234a2ce378e85baad2d0499c60dbc3037b29da2cf4d4e796e52a6fd264d570000000000ffffffff0200e1f50500000000220020421305d3c9f5faee4f8fa7dc79f54c5eb2971ff4819cfac68fcb219c14f8aed5fb8d435500000000160014f1a65dab1d878c0ae057a5d3cc1b7d31f494d774024830450221008e9582cf252ed3b9f05c38a017c675ae8c55f0aba3bc9c1dfddb0fbbe3b17c3f022002d4c28e687597aa89445a78afdee34d96852493c140b573fa8b157edc3a704901210250e42d3f14ed353ffb49c8a041bab867dadbbb987a41007c7424aa7c5cad90ca00000000

{
  "hash": "427801ac614d0bc8195aea7170f7c5963b0d88edd9e6e885132ca3cc5a6ff3de",
  "locktime": 0,
  "size": 235,
  "txid": "cd67afd1602424c42cd2b1f0d711600978cb17b1bf045103893002b3b8c52c5a",
  "version": 2,
  "vin": [
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967295,
      "txid": "574d26fda6526e794e4dcfa29db23730bc0dc699042dadba858e37cea23422d6",
      "txinwitness": [
        "30450221008e9582cf252ed3b9f05c38a017c675ae8c55f0aba3bc9c1dfddb0fbbe3b17c3f022002d4c28e687597aa89445a78afdee34d96852493c140b573fa8b157edc3a704901",
        "0250e42d3f14ed353ffb49c8a041bab867dadbbb987a41007c7424aa7c5cad90ca"
      ],
      "vout": 0
    }
  ],
  "vout": [
    {
      "n": 0,
      "scriptPubKey": {
        "address": "bc1qggfst57f7hawunu05lw8na2vt6efw8l5sxw04350evsec98c4m2sjy726s",
        "asm": "0 421305d3c9f5faee4f8fa7dc79f54c5eb2971ff4819cfac68fcb219c14f8aed5",
        "desc": "addr(bc1qggfst57f7hawunu05lw8na2vt6efw8l5sxw04350evsec98c4m2sjy726s)#kv9y7ult",
        "hex": "0020421305d3c9f5faee4f8fa7dc79f54c5eb2971ff4819cfac68fcb219c14f8aed5",
        "type": "witness_v0_scripthash"
      },
      "value": 1.0
    },
    {
      "n": 1,
      "scriptPubKey": {
        "address": "bc1q7xn9m2cas7xq4czh5hfucxmax86ff4m5t59tz5",
        "asm": "0 f1a65dab1d878c0ae057a5d3cc1b7d31f494d774",
        "desc": "addr(bc1q7xn9m2cas7xq4czh5hfucxmax86ff4m5t59tz5)#zfruqa4k",
        "hex": "0014f1a65dab1d878c0ae057a5d3cc1b7d31f494d774",
        "type": "witness_v0_keyhash"
      },
      "value": 14.30490619
    }
  ],
  "vsize": 153,
  "weight": 610
}
```

### Spend from p2wsh

```sh
$ bitcoin-cli decoderawtransaction 020000000001015a2cc5b8b3023089035104bfb117cb78096011d7f0b1d22cc4242460d1af67cd0000000000ffffffff020000000000000000106a0e4d617474686577205a69706b696e18ddf50500000000160014f1a65dab1d878c0ae057a5d3cc1b7d31f494d77404004730440220311e223e3582608d45d1563774ebafe08f9c31c5ade6b1e553a2cfe803b59ab3022012929a2af9a56e8cbc13cf5df4eb5b2029ec2434835d49661d53b8ff183c4e06014730440220137d76b3701ecdbe45434cad77aecca251352141d5023d3fbe8b7ba33249852a022033ad7ffa4426b8e952a81aa097a9fd4610095cb6f801b3c5f61349ec7557db4001475221034837d61d3fef06d5eef3cd6fb1d52475afcfc933df442d15ead33bf963994ebb210277a243e2dd32d5425e160a7e5f5765e659556ff11f252fe4a10185038d3ed51e52ae00000000

{
  "hash": "2f0ff94b6a382f087630e2f4030357309e9f5eb6df87fadc8dee59d394e6160a",
  "locktime": 0,
  "size": 327,
  "txid": "3c15a90cc39cedf83d3dabaf2370591116be6467f3154c665d8b25871921a7bb",
  "version": 2,
  "vin": [
    {
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "sequence": 4294967295,
      "txid": "cd67afd1602424c42cd2b1f0d711600978cb17b1bf045103893002b3b8c52c5a",
      "txinwitness": [
        "",
        "30440220311e223e3582608d45d1563774ebafe08f9c31c5ade6b1e553a2cfe803b59ab3022012929a2af9a56e8cbc13cf5df4eb5b2029ec2434835d49661d53b8ff183c4e0601",
        "30440220137d76b3701ecdbe45434cad77aecca251352141d5023d3fbe8b7ba33249852a022033ad7ffa4426b8e952a81aa097a9fd4610095cb6f801b3c5f61349ec7557db4001",
        "5221034837d61d3fef06d5eef3cd6fb1d52475afcfc933df442d15ead33bf963994ebb210277a243e2dd32d5425e160a7e5f5765e659556ff11f252fe4a10185038d3ed51e52ae"
      ],
      "vout": 0
    }
  ],
  "vout": [
    {
      "n": 0,
      "scriptPubKey": {
        "asm": "OP_RETURN 4d617474686577205a69706b696e",
        "desc": "raw(6a0e4d617474686577205a69706b696e)#f8w4cxgd",
        "hex": "6a0e4d617474686577205a69706b696e",
        "type": "nulldata"
      },
      "value": 0.0
    },
    {
      "n": 1,
      "scriptPubKey": {
        "address": "bc1q7xn9m2cas7xq4czh5hfucxmax86ff4m5t59tz5",
        "asm": "0 f1a65dab1d878c0ae057a5d3cc1b7d31f494d774",
        "desc": "addr(bc1q7xn9m2cas7xq4czh5hfucxmax86ff4m5t59tz5)#zfruqa4k",
        "hex": "0014f1a65dab1d878c0ae057a5d3cc1b7d31f494d774",
        "type": "witness_v0_keyhash"
      },
      "value": 0.99999
    }
  ],
  "vsize": 162,
  "weight": 648
}

```
