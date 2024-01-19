#![allow(unused)]
extern crate balance;
use balance::WalletState;

#[derive(Debug)]
pub enum SpendError {
    MissingCodeCantRun,
    // Add more relevant error variants
}

pub struct Utxo {
    script_pubkey: Vec<u8>,
    amount: u32,
}

pub struct Outpoint {
    txid: [u8; 32],
    index: u32,
}

// Given 2 compressed public keys as byte arrays, construct
// a 2-of-2 multisig output script. No length byte prefix is necessary.
fn create_multisig_script(keys: Vec<Vec<u8>>) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given an output script as a byte array, compute the p2wsh witness program
// This is a segwit version 0 pay-to-script-hash witness program.
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#p2wsh
fn get_p2wsh_program(script: &[u8], version: Option<u32>) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given an outpoint, return a serialized transaction input spending it
// Use hard-coded defaults for sequence and scriptSig
fn input_from_utxo(txid: &[u8], index: u32) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given an output script and value (in satoshis), return a serialized transaction output
fn output_from_options(script: &[u8], value: u32) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given a Utxo object, extract the public key hash from the output script
// and assemble the p2wpkh scriptcode as defined in BIP143
// <script length> OP_DUP OP_HASH160 <pubkey hash> OP_EQUALVERIFY OP_CHECKSIG
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
fn get_p2wpkh_scriptcode(utxo: Utxo) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Compute the commitment hash for a single input and return bytes to sign.
// This implements the BIP 143 transaction digest algorithm
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
// We assume only a single input and two outputs,
// as well as constant default values for sequence and locktime
fn get_commitment_hash(
    outpoint: Outpoint,
    scriptcode: &[u8],
    value: u32,
    outputs: Vec<Utxo>,
) -> Vec<u8> {
    // Version

    // All TX input outpoints (only one in our case)

    // All TX input sequences (only one for us, always default value)

    // Single outpoint being spent

    // Scriptcode (the scriptPubKey in/implied by the output being spent, see BIP 143)

    // Value of output being spent

    // Sequence of output being spent (always default for us)

    // All TX outputs

    // Locktime (always default for us)

    // SIGHASH_ALL (always default for us)

    unimplemented!("implement the logic")
}

// Given a JSON utxo object and a list of all of our wallet's witness programs,
// return the index of the derived key that can spend the coin.
// This index should match the corresponding private key in our wallet's list.
fn get_key_index(utxo: Utxo, programs: Vec<&str>) -> u32 {
    unimplemented!("implement the logic")
}

// Given a private key and message digest as bytes, compute the ECDSA signature.
// Bitcoin signatures:
// - Must be strict-DER encoded
// - Must have the SIGHASH_ALL byte (0x01) appended
// - Must have a low s value as defined by BIP 62:
//   https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki#user-content-Low_S_values_in_signatures
fn sign(privkey: &[u8; 32], msg: Vec<u8>) -> Vec<u8> {
    // Keep signing until we produce a signature with "low s value"
    // We will have to decode the DER-encoded signature and extract the s value to check it
    // Format: 0x30 [total-length] 0x02 [R-length] [R] 0x02 [S-length] [S] [sighash]
    unimplemented!("implement the logic")
}

// Given a private key and transaction commitment hash to sign,
// compute the signature and assemble the serialized p2pkh witness
// as defined in BIP 141 (2 stack items: signature, compressed public key)
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#specification
fn get_p2wpkh_witness(privkey: &[u8; 32], msg: Vec<u8>) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given two private keys and a transaction commitment hash to sign,
// compute both signatures and assemble the serialized p2pkh witness
// as defined in BIP 141
// Remember to add a 0x00 byte as the first witness element for CHECKMULTISIG bug
// https://github.com/bitcoin/bips/blob/master/bip-0147.mediawiki
fn get_p2wsh_witness(privs: Vec<&[u8; 32]>, msg: Vec<u8>) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given arrays of inputs, outputs, and witnesses, assemble the complete
// transaction and serialize it for broadcast. Return bytes as hex-encoded string
// suitable to broadcast with Bitcoin Core RPC.
// https://en.bitcoin.it/wiki/Protocol_documentation#tx
fn assemble_transaction(
    inputs: Vec<Vec<u8>>,
    outputs: Vec<Utxo>,
    witnesses: Vec<Vec<u8>>,
) -> Vec<u8> {
    unimplemented!("implement the logic")
}

// Given arrays of inputs and outputs (no witnesses!) compute the txid.
// Return the 32 byte txid as a *reversed* hex-encoded string.
// https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format
fn get_txid(inputs: Vec<Vec<u8>>, outputs: Vec<Utxo>) -> [u8; 32] {
    unimplemented!("implement the logic")
}

// Spend a p2wpkh utxo to a 2 of 2 multisig p2wsh and return the (txid, transaction) tupple
pub fn spend_p2wpkh(wallet_state: &WalletState) -> Result<([u8; 32], Vec<u8>), SpendError> {
    // FEE = 1000
    // AMT = 1000000
    // Choose an unspent coin worth more than 0.01 BTC

    // Create the input from the utxo
    // Reverse the txid hash so it's little-endian

    // Compute destination output script and output

    // Compute change output script and output

    // Get the message to sign

    // Fetch the private key we need to sign with

    // Sign!

    // Assemble

    // Reserialize without witness data and double-SHA256 to get the txid

    // For debugging you can use RPC `testmempoolaccept ["<final hex>"]` here

    // return txid, final-tx

    unimplemented!("implement the logic")
}

// Spend a 2-of-2 multisig p2wsh utxo and return the transaction
pub fn spend_p2wsh(wallet_state: &WalletState, txid: [u8; 32]) -> Result<Vec<Vec<u8>>, SpendError> {
    // COIN_VALUE = 1000000
    // FEE = 1000
    // AMT = 0
    // Create the input from the utxo
    // Reverse the txid hash so it's little-endian

    // Compute destination output script and output

    // Compute change output script and output

    // Get the message to sign

    // Sign!

    // Assemble

    // For debugging you can use RPC `testmempoolaccept ["<final hex>"]` here
    // return txid final-tx

    unimplemented!("implement the logic")
}
