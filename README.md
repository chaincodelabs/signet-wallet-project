# Signet Wallet Project

The goal of this project is to write a simple wallet over two weeks and use it
to interact with a custom signet network provided by the administrator.

## Simplify

To reduce the scope of this project the wallet will be very limited:
- No separate change addresses: one descriptor is used for all internal and external addressees.
- No [VarInt](https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer):
all vectors will be under 255 bytes in length and always require one single
byte to indicate length.
- All sending and receiving addresses will be [`p2wpkh`](https://en.bitcoin.it/wiki/BIP_0141#P2WPKH)
    - Except one [`p2wsh`](https://en.bitcoin.it/wiki/BIP_0141#P2WSH) multisig which is the goal of the week 2 assignment
- Fees can be hard-coded by value, no estimation is necessary.
- Transactions you create will always have exactly 1 input and 2 outputs.
- Don't worry about invalid keys (probabilty is less than 1 in 2<sup>127</sup>)
- Other constants:
    - All transactions will be version 2 (little-endian encoded as `\x02\x00\x00\x00`)
    - All input sequences will be `0xffffffff`
    - All transaction locktimes will be `0x00000000`
    - All input scriptSigs will be `0x00` (because we are only concerned with segregated witness inputs)
    - All sighash flags will be `SIGHASH_ALL` which is `0x01` in signatures and little-endian encoded as `\x01\x00\x00\x00` in transaction commitments

## Custom Signet

We will pre-fabricate a signet blockchain and host a mining node that everyone
can connect to, download the chain (around 20 MB), and broadcast their completed transactions.
We will have already generated private key descriptors for each student and used
them to generate hundreds of transactions sending and receiving in the chain.
Each student will be provided with a single `wpkh` descriptor with an extended
private key like this example:

`wallet_000: wpkh(tprv8ZgxMBicQKsPekvUvQEdWuFnTazwP6QwU15RJCTAEUaUn9ti3NEtywCywj1PF4G2MzvTad4F3MSSZvT2nZuxHBJ9HaZad5r1dYJtzYsR9iW/84h/1h/0h/0/*)#3ajpxxju`

The important elements here are the extended private key (`tprv...`) and the
derivation path (`84h/1h/0h/0/*`).

*Note:* If you have already connected to the "default" (or any other) signet network
with your node, you may need to rename / move your existing data directory. See
[#27494](https://github.com/bitcoin/bitcoin/issues/27494)

A `bitcoin.conf` file will be provided to students in [config/bitcoin.conf](config/bitcoin.conf)
which will set the address of the mining node as well as provide the signet
[challenge](https://en.bitcoin.it/wiki/Signet). Copy this file to your system's
[default datadir location](https://github.com/bitcoin/bitcoin/blob/master/doc/bitcoin-conf.md#configuration-file-path)
and start Bitcoin Core:

`bitcoind -signet`

You should also then be able to
execute RPCs with (for example):

`bitcoin-cli -signet getblockcount`

# Week 1 (keys and addresses)

See [Recover Balance](./recover_balance.md) coding challenge

# Week 2 (transactions, scripts, and signatures)

See [Send Multisig](./send_multisig.md) coding challenge

# Admin Infrastructure

**Students do not need to read this section**

<details>
    <summary>Setup the Signet server</summary>

The included script [signet-setup.py](./signet-setup.py) needs to be run by
the administrator on a publicly reachable server to start the game.

The script requires a local installation of Bitcoin Core since it consumes
the test framework as a library.

Usage: `python signet-setup.py <path/to/bitcoin> <path/to/student/files> <path/for/bitcoin/datadir>`

`<path/to/bitcoin>`: (required) Path to local installation of Bitcoin Core repository

`<path/to/student/files>`: (optional, default `./config`) Destination for student bitcoin.conf and wallet descriptors

`<path/for/bitcoin/datadir>`: (optional, default is `os.tmpdir()`) Data directory for the signet full node

The script runs the signet full node, creates all the wallets and continues mining blocks forever.
It should never be killed, but the node can always be restarted by using `-datadir=<path/for/bitcoin/datadir>`

</details>
