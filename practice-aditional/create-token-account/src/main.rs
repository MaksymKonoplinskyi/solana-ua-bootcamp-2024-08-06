use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account};
use solana_client::rpc_request::TokenAccountsFilter;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok(); 
    println!("Завантажено .env файл");
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format!");
    let sender = Keypair::from_bytes(&as_array).expect("Failed to create keypair from secret key");

    let connection = RpcClient::new_with_commitment(cluster_api_url("devnet"), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str("6KEmjrDXwez9azPNL3t4XP7PieZyhsG8WnvF6ihgRswb")
        .expect("Invalid token mint account");
    let recipient = Pubkey::from_str("Ho4zSxj1XobiqbajbENs34b1kqFsjQrUK5jVzc2PP22")
        .expect("Invalid recipient public key");

    let token_account = get_or_create_associated_token_account(
        &connection,
        &sender,
        &token_mint_account,
        &recipient,
    );

    println!("Token Account: {}", token_account.to_string());

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_account.to_string()
    );

    println!("✅ Created token account: {}", link);
}

fn get_or_create_associated_token_account(
    connection: &RpcClient,
    sender: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Pubkey {
    let associated_token_address = get_associated_token_address(owner, mint);

    let account = connection
        .get_token_accounts_by_owner(owner, TokenAccountsFilter::ProgramId(spl_token::id()))
        .expect("Failed to get token accounts");

    if account.is_empty() {
        let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[create_associated_token_account(
                &sender.pubkey(),
                owner,
                mint,
                &spl_associated_token_account::id(),
            )],
            Some(&sender.pubkey()),
            &[sender],
            recent_blockhash,
        );

        connection
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        associated_token_address
    } else {
        associated_token_address
    }
}

fn cluster_api_url(cluster: &str) -> &str {
    match cluster {
        "devnet" => "https://api.devnet.solana.com",
        _ => panic!("Unsupported cluster"),
    }
}

