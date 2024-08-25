use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    program_pack::Pack, // Імпортуємо трейt Pack
};
use solana_client::rpc_client::RpcClient;
use spl_token::{instruction::initialize_mint, state::Mint};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");

    let as_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_vec).expect("Failed to create Keypair");

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );

    println!("🔑 Our public key is: {}", sender.pubkey());

    // Використаємо той же Keypair для mint
    let token_mint = Keypair::new();

    println!("Token Mint Public Key: {}", token_mint.pubkey());

    // Отримуємо мінімальну ренту для акаунту mint
    let rent_exemption = connection
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("Failed to get rent exemption");

    // Створюємо рахунок mint
    let create_mint_account_ix = system_instruction::create_account(
        &sender.pubkey(),
        &token_mint.pubkey(),
        rent_exemption,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    // Інструкція для ініціалізації mint
    let initialize_mint_ix = initialize_mint(
        &spl_token::id(),
        &token_mint.pubkey(),
        &sender.pubkey(),
        None,
        2,
    )
    .unwrap();

    let recent_blockhash = match connection.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(e) => {
            eprintln!("Failed to get latest blockhash: {:?}", e);
            return;
        }
    };

    // Підписання транзакції з правильними Keypair
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, initialize_mint_ix],
        Some(&sender.pubkey()),   // Обов'язково використати правильний підписувач
        &[&sender, &token_mint],   // Підписуємо транзакцію від імені sender та mint
        recent_blockhash,
    );

    match connection.send_and_confirm_transaction_with_spinner(&transaction) {
        Ok(_) => {
            let explorer_link = format!(
                "https://explorer.solana.com/address/{}?cluster=devnet",
                token_mint.pubkey().to_string()
            );
            println!("✅ Token Mint: {}", explorer_link);
        },
        Err(e) => {
            eprintln!("Transaction failed: {:?}", e);
        }
    }
}
