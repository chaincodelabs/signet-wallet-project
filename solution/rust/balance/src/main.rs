use std::env;

fn main() {
    // Set up for RPC client on local and remote CI server
    let args: Vec<String> = env::args().collect();
    
    // Default Bitcoin Core cookie path
    let mut cookie_filepath = r"~/.bitcoin/signet/.cookie";
    
    if args.len() > 1 {
        cookie_filepath = &args[1];
    }
    
    // Run the program in lib.rs and print any errors
    if let Err(e) = balance::run(cookie_filepath) {
        eprintln!("{}", e);
    }
}
