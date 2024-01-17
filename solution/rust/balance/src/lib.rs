extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bs58;
use hex;
use hmac_sha512::HMAC;
use num_bigint::BigUint; // for modulus math on large numbers
use ripemd::{Ripemd160};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::str;

// Provided by administrator
const WALLET_NAME: &str = "wallet_000";
const EXTENDED_PRIVATE_KEY: &str = tprv8ZgxMBicQKsPfCxvMSGLjZegGFnZn9VZfVdsnEbuzTGdS9aZjvaYpyh7NsxsrAc8LsRQZ2EYaCfkvwNpas8cKUBbptDzadY7c3hUi8i33XJ
const HARDENED_OFFSET: u32 = 2_u32.pow(31);

struct ExtendedKey {

}

struct ChildKey {

}

pub struct OutgoingTx {

}

struct SpendingTx {

}

// final wallet state struct
pub struct WalletState {
    utxos: Vec<OutgoingTx>,
    witness_programs: Vec<[u8; 22]>,
    public_keys: Vec<[u8; 33]>,
    private_keys: Vec<[u8; 32]>,
}

// Decode a base58 string into an array of bytes
fn base58_decode(base58_string: &str) -> Vec<u8> {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    // Convert Base58 string to a big integer

    // Convert the integer to bytes

    // Chop off the 32 checksum bits and return

    // BONUS POINTS: Verify the checksum!
}

// Deserialize the extended key bytes and return a JSON object
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
// 4 byte: version bytes (mainnet: 0x0488B21E public, 0x0488ADE4 private; testnet: 0x043587CF public, 0x04358394 private)
// 1 byte: depth: 0x00 for master nodes, 0x01 for level-1 derived keys, ....
// 4 bytes: the fingerprint of the parent's key (0x00000000 if master key)
// 4 bytes: child number. This is ser32(i) for i in xi = xpar/i, with xi the key being serialized. (0x00000000 if master key)
// 32 bytes: the chain code
// 33 bytes: the public key or private key data (serP(K) for public keys, 0x00 || ser256(k) for private keys)
fn deserialize_key(bytes: Vec<u8>) -> ExtendedKey {    

}

// Derive the secp256k1 compressed public key from a given private key
// BONUS POINTS: Implement ECDSA yourself and multiply you key by the generator point!
fn derive_public_key_from_private(key: &[u8; 32]) -> [u8; 33] {

}

// Perform a BIP32 parent private key -> child private key operation
// Return a JSON object with "key" and "chaincode" properties as bytes
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Private_parent_key_rarr_private_child_key
fn derive_priv_child(key: &[u8; 32], chaincode: &[u8; 32], index: u32) -> ChildKey {

}

// Given an extended private key and a BIP32 derivation path, compute the child private key found at the last path
// The derivation path is formatted as an array of (index: int, hardened: bool) tuples.
fn get_child_key_at_path(key: [u8; 32], chaincode: [u8; 32], paths: Vec<(u32, bool)>) -> ChildKey {

}

// Compute the first N child private keys.
// Return an array of keys encoded as bytes.
fn get_keys_at_child_key_path(child_key: ChildKey, num_keys: u32) -> Vec<[u8; 32]> {

}

// Derive the p2wpkh witness program (aka scriptPubKey) for a given compressed public key.
// Return a bytes array to be compared with the JSON output of Bitcoin Core RPC getblock
// so we can find our received transactions in blocks.
// These are segwit version 0 pay-to-public-key-hash witness programs.
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WPKH
fn get_p2wpkh_program(pubkey: [u8; 33]) -> [u8; 22] {

}

// public function that will be called by `run` here as well as the spend program externally
pub fn recover_wallet_state(extended_private_key: &str, cookie_filepath: &str) -> Result<WalletState, Box<dyn Error>> {
    // Deserialize the provided extended private key

    // Derive the key and chaincode at the path in the descriptor (`84h/1h/0h/0`)
   
    // Get the child key at the derivation path

    // Compute 2000 private keys from the child key path
    // For each private key, collect compressed public keys and witness programs
    let private_keys = vec![];
    let public_keys = vec![];
    let witness_programs = vec![];

    // Collect outgoing and spending txs from a block scan
    let mut outgoing_txs: Vec<OutgoingTx> = vec![];
    let mut spending_txs: Vec<SpendingTx> = vec![];
    let mut utxos: Vec<OutgoingTx> = vec![];

    // set up bitcoin-core-rpc on signet
    let path = PathBuf::from(cookie_filepath);
    let rpc = Client::new("http://localhost:38332", Auth::CookieFile(path))?;

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

pub fn run(rpc_cookie_filepath: &str) -> Result<(), ()> {
    let utxos = recover_wallet_state(EXTENDED_PRIVATE_KEY, rpc_cookie_filepath)?;
    let balance:

    println!("{} {:.8}", WALLET_NAME, balance);
    Ok(())
}
