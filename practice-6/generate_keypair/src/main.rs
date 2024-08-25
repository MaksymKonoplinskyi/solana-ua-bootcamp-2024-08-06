use solana_sdk::signature::{Keypair, Signer};
use base64::{encode};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let mut keypair = Keypair::new();
    let mut public_key_base58 = keypair.pubkey().to_string();

    while !public_key_base58.starts_with("M") {
        keypair = Keypair::new();
        public_key_base58 = keypair.pubkey().to_string();
    }

    let secret_key_base64 = encode(keypair.to_bytes());

    let time_taken = start_time.elapsed().as_secs_f64();

    println!("The public key is: {}", public_key_base58);
    println!("The secret key is: {}", secret_key_base64);
    println!("Time taken: {} seconds", time_taken);
    println!("✅ Finished!");
}
