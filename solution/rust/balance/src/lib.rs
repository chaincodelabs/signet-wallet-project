#![allow(unused)]
use std::path::PathBuf;

// Import necessary bitcoin primitives
extern crate bitcoin;
use bitcoin::{
    bip32::{Xpriv, Xpub},
    script::PushBytesBuf,
    Amount, PrivateKey, PublicKey, ScriptBuf, Transaction, TxOut, WitnessProgram,
};

// Import core rpc client utils
extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client};

// Provided by administrator
pub const WALLET_NAME: &str = "wallet_000";
pub const EXTENDED_PRIVATE_KEY: &str = "tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ";

#[derive(Debug)]
pub enum BalanceError {
    MissingCodeCantRun,
    // Add relevant error variants for various cases.
}
// final wallet state struct
pub struct WalletState {
    utxos: Vec<TxOut>,
    witness_programs: WitnessProgram,
    public_keys: Vec<PublicKey>,
    private_keys: Vec<PrivateKey>,
}

impl WalletState {
    // Given a WalletState find the balance
    pub fn balance(&self) -> Amount {
        unimplemented!("implement the logic")
    }
}

// Decode a base58 string into an array of bytes
fn base58_decode(base58_string: &str) -> Vec<u8> {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    // Convert Base58 string to a big integer

    // Convert the integer to bytes

    // Chop off the 32 checksum bits and return

    // BONUS POINTS: Verify the checksum!
    unimplemented!("implement the logic")
}

// Deserialize the extended pubkey bytes and return a Xpub object
// Definition of the Xpub rust struct: https://docs.rs/bitcoin/0.31.1/bitcoin/bip32/struct.Xpub.html
// Bip32 Serialization format: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
fn deserialize_xpub(bytes: &[u8]) -> Xpub {
    unimplemented!("implement the logic")
}

// Deserialize the extended pubkey bytes and return a Xpriv object
// Definition of the Xpub rust struct: https://docs.rs/bitcoin/0.31.1/bitcoin/bip32/struct.Xpriv.html
// Bip32 Serialization format: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
fn deserialize_xpriv(bytes: &[u8]) -> Xpriv {
    unimplemented!("implement the logic")
}

// Derive the secp256k1 compressed public key from a given private key
// BONUS POINTS: Implement ECDSA yourself and multiply you key by the generator point!
fn derive_public_key_from_private(key: &PrivateKey) -> PublicKey {
    unimplemented!("implement the logic")
}

// Perform a BIP32 parent private key -> child private key derivation
// Return a derived child Xpriv, given a child_number. Check the struct docs for APIs.
// Key derivation steps: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Private_parent_key_rarr_private_child_key
fn derive_priv_child(key: Xpriv, child_num: u32) -> Xpriv {
    unimplemented!("implement the logic")
}

// Given an extended private key and a BIP32 derivation path, compute the child private key found at the path
// Derivation paths are strings like "m/0'/1/2h/2"
fn get_child_key_at_path(key: Xpriv, derivation_path: &str) -> Xpriv {
    unimplemented!("implement the logic")
}

// Compute the first N child private keys.
// Return an array of keys.
fn get_keys_at_child_key_path(child_key: Xpriv, num_keys: u32) -> Vec<Xpriv> {
    unimplemented!("implement the logic")
}

// Derive the p2wpkh witness program (aka scriptPubKey) for a given compressed public key
// Return a bytes array to be compared with the JSON output of Bitcoin Core RPC getblock
// so we can find our received transactions in blocks
// These are segwit version 0 pay-to-public-key-hash witness programs
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WPKH
fn get_p2wpkh_program(pubkey: PublicKey) -> ScriptBuf {
    unimplemented!("implement the logic")
}

// public function that will be called by `run` here as well as the spend program externally
pub fn recover_wallet_state(
    extended_private_key: &str,
    cookie_filepath: &str,
) -> Result<WalletState, BalanceError> {
    // Deserialize the provided extended private key

    // Derive the key and chaincode at the path in the descriptor (`84h/1h/0h/0`)

    // Get the child key at the derivation path

    // Compute 2000 private keys from the child key path
    // For each private key, collect compressed public keys and witness programs
    let private_keys = vec![];
    let public_keys = vec![];
    let witness_programs = WitnessProgram::new(bitcoin::WitnessVersion::V1, PushBytesBuf::new())
        .map_err(|_| BalanceError::MissingCodeCantRun)?;

    // Collect outgoing and spending txs from a block scan
    let mut outgoing_txs: Vec<Transaction> = vec![];
    let mut spending_txs: Vec<Transaction> = vec![];
    let mut utxos: Vec<TxOut> = vec![];

    // set up bitcoin-core-rpc client on signet, by providing the rpc url and path to the cookie file.
    // Change the values according to your local bitcoind setup.
    let rpc = Client::new(
        "local-host:signet-rpc-port",
        Auth::CookieFile("cookie/file/path".into()),
    )
    .map_err(|_| BalanceError::MissingCodeCantRun)?;

    // Scan blocks 0 to 300 for transactions
    // Check every tx input (witness) for our own compressed public keys. These are coins we have spent.
    // Check every tx output for our own witness programs. These are coins we have received.
    // Keep track of outputs by their outpoint so we can check if it was spent later by an input
    // Collect outputs that have not been spent into a utxo set
    // Return Wallet State
    Ok(WalletState {
        utxos,
        public_keys,
        private_keys,
        witness_programs,
    })
}
