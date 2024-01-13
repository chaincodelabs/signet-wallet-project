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

const WALLET_NAME: &str = "";
const EXTENDED_PRIVATE_KEY: &str = "";
const HARDENED_OFFSET: u32 = 2_u32.pow(31);

struct ExtendedKey {
    key: [u8; 33],
    chaincode: [u8; 32],
}

struct ChildKey {
    key: [u8; 32],
    chaincode: [u8; 32],
}

// structures for capturing transactions from block scan
pub struct OutgoingTx {
    outpoint: [u8; 36],
    witness_program: [u8; 22],
    value: f64,
}
struct SpendingTx {
    outpoint: [u8; 36],
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
    vec![]
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
    ExtendedKey {
        key: [0; 33],
        chaincode: [0; 32],
    }
}

// Derive the secp256k1 compressed public key from a given private key
// BONUS POINTS: Implement ECDSA yourself and multiply you key by the generator point!
fn derive_public_key_from_private(key: &[u8; 32]) -> [u8; 33] {
    [0; 33]
}

// Perform a BIP32 parent private key -> child private key operation
// Return a JSON object with "key" and "chaincode" properties as bytes
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Private_parent_key_rarr_private_child_key
fn derive_priv_child(key: &[u8; 32], chaincode: &[u8; 32], index: u32) -> ChildKey {
    ChildKey {
        key: [0; 32],
        chaincode: [0; 32],
    }
}

// Given an extended private key and a BIP32 derivation path, compute the child private key found at the last path
// The derivation path is formatted as an array of (index: int, hardened: bool) tuples.
fn get_child_key_at_path(key: [u8; 32], chaincode: [u8; 32], paths: Vec<(u32, bool)>) -> ChildKey {
    ChildKey {
        key: [0; 32],
        chaincode: [0; 32],
    }
}

// Compute the first N child private keys.
// Return an array of keys encoded as bytes.
fn get_keys_at_child_key_path(child_key: ChildKey, num_keys: u32) -> Vec<[u8; 32]> {
    vec![]
}

// Derive the p2wpkh witness program (aka scriptPubKey) for a given compressed public key.
// Return a bytes array to be compared with the JSON output of Bitcoin Core RPC getblock
// so we can find our received transactions in blocks.
// These are segwit version 0 pay-to-public-key-hash witness programs.
// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WPKH
fn get_p2wpkh_program(pubkey: [u8; 33]) -> Vec<u8> {
    vec![]
}

// public function that will be called by `run` here as well as the spend program externally
pub fn recover_wallet_state(extended_private_key: &str, cookie_filepath: &str) -> Result<WalletState, Box<dyn Error>> {
    // Deserialize the provided extended private key

    // Derive the key and chaincode at the path in the descriptor (`84h/1h/0h/0`)
   
    // Get the child key at the derivation path

    // Compute 2000 private keys from the child key path
    let private_keys = vec![];
    let public_keys = vec![];
    let witness_programs = vec![];

    // For each private key, collect compressed public keys and witness programs

    // Collect outgoing and spending txs from a block scan
    let mut outgoing_txs: Vec<OutgoingTx> = vec![];
    let mut spending_txs: Vec<SpendingTx> = vec![];
    let mut utxos: Vec<OutgoingTx> = vec![];

    // start up bitcoin-core-rpc
    let path = PathBuf::from(cookie_filepath);
    let rpc = Client::new("http://localhost:38332", Auth::CookieFile(path))?;

    // Scan blocks 0 to 300 for transactions
    // Check every tx input (witness) for our own compressed public keys. These are coins we have spent.
    // Check every tx output for our own witness programs. These are coins we have received.
    // Keep track of UTXOs by outpoint so we can check if it was spent later
    // Return UTXOs
    Ok(WalletState {
        utxos,
        public_keys,
        private_keys,
        witness_programs,
    })
}

pub fn run(rpc_cookie_filepath: &str) -> Result<(), Box<dyn Error>> {
    let utxos = recover_wallet_state(EXTENDED_PRIVATE_KEY, rpc_cookie_filepath)?;
    let balance: f64 = 0.0;

    println!("{} {:.8}", WALLET_NAME, balance);
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use super::base58_decode;
    use super::get_child_key_at_path;
    use super::deserialize_key;
    use super::get_p2wpkh_program;

    #[test]
    // Test vectors from https://en.bitcoin.it/wiki/BIP_0032_TestVectors
    fn test_child_key_at_path() {
        // Derivation Path: [Chain m]
        let extended_private_key = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";
        let deserialized_key = deserialize_key(base58_decode(extended_private_key));
        let private_key: [u8; 32] = deserialized_key.key[1..].try_into().unwrap();
        let chaincode = deserialized_key.chaincode;
        assert_eq!(hex::encode(private_key), "e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35");
        assert_eq!(hex::encode(chaincode), "873dff81c02f525623fd1fe5167eac3a55a049de3d314bb42ee227ffed37d508");

        // Derivation Path: [Chain m/0']
        let derivation_path: Vec<(u32, bool)> = vec![(0, true)];
        let child_key = get_child_key_at_path(private_key, chaincode, derivation_path);
        assert_eq!(hex::encode(child_key.key), "edb2e14f9ee77d26dd93b4ecede8d16ed408ce149b6cd80b0715a2d911a0afea");
        assert_eq!(hex::encode(child_key.chaincode), "47fdacbd0f1097043b78c63c20c34ef4ed9a111d980047ad16282c7ae6236141");

        // Derivation Path: [Chain m/0'/1]
        let derivation_path: Vec<(u32, bool)> = vec![(0, true), (1, false)];
        let child_key = get_child_key_at_path(private_key, chaincode, derivation_path);
        assert_eq!(hex::encode(child_key.key), "3c6cb8d0f6a264c91ea8b5030fadaa8e538b020f0a387421a12de9319dc93368");
        assert_eq!(hex::encode(child_key.chaincode), "2a7857631386ba23dacac34180dd1983734e444fdbf774041578e9b6adb37c19");

        // Derivation Path: [Chain m/0'/1/2']
        let derivation_path: Vec<(u32, bool)> = vec![(0, true), (1, false), (2, true)];
        let child_key = get_child_key_at_path(private_key, chaincode, derivation_path);
        assert_eq!(hex::encode(child_key.key), "cbce0d719ecf7431d88e6a89fa1483e02e35092af60c042b1df2ff59fa424dca");
        assert_eq!(hex::encode(child_key.chaincode), "04466b9cc8e161e966409ca52986c584f07e9dc81f735db683c3ff6ec7b1503f");
    }

    #[test]
    // p2wpkh witness program
    // https://learnmeabitcoin.com/technical/public-key-hash
    fn test_pubkey_to_witness_program() {
        let public_key: [u8; 33] = hex::decode("02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737").unwrap().try_into().unwrap(); // compressed public key
        let program = get_p2wpkh_program(public_key);
        assert_eq!(hex::encode(&program), "001493ce48570b55c42c2af816aeaba06cfee1224fae")
    }
}
