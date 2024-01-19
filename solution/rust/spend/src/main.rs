extern crate balance;
use balance::{recover_wallet_state, EXTENDED_PRIVATE_KEY};

use spend::{spend_p2wpkh, spend_p2wsh};
fn main() {
    // Default Bitcoin Core cookie path
    let cookie_filepath = "~/.bitcoin/signet/.cookie";

    let wallet_state = recover_wallet_state(EXTENDED_PRIVATE_KEY, cookie_filepath).unwrap();
    let (txid1, tx1) = spend_p2wpkh(&wallet_state).unwrap();
    println!("tx1: {:?}", tx1);
    let tx2 = spend_p2wsh(&wallet_state, txid1).unwrap();
    println!("tx2: {:?}", tx2);
}
