use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};
use spl_token::instruction::mint_to;
use std::str::FromStr;
use std::env;
use serde_json;

const MINOR_UNITS_PER_MAJOR_UNITS: u64 = 10u64.pow(2);

fn get_explorer_link(transaction_signature: &str, cluster: &str) -> String {
    match cluster {
        "devnet" => format!("https://explorer.solana.com/tx/{}?cluster=devnet", transaction_signature),
        "testnet" => format!("https://explorer.solana.com/tx/{}?cluster=testnet", transaction_signature),
        "mainnet" => format!("https://explorer.solana.com/tx/{}?cluster=mainnet-beta", transaction_signature),
        _ => format!("https://explorer.solana.com/tx/{}", transaction_signature),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let private_key = env::var("SECRET_KEY")?;
    let as_vec: Vec<u8> = serde_json::from_str(&private_key)?;
    let sender = Keypair::from_bytes(&as_vec).expect("Failed to create Keypair");

    let url = "https://api.devnet.solana.com";
    let connection = RpcClient::new(url.to_string());

    let token_mint_account = Pubkey::from_str("6KEmjrDXwez9azPNL3t4XP7PieZyhsG8WnvF6ihgRswb")?;
    let recipient_associated_token_account = Pubkey::from_str("4CvFf3KfbTBeL7zte3X3r5uWkFyP2gdv1GGMadm5Kf2f")?;

    let instruction = mint_to(
        &sender.pubkey(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * MINOR_UNITS_PER_MAJOR_UNITS,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let transaction_signature = connection.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?;

    let transaction_signature_str = transaction_signature.to_string();
    let link = get_explorer_link(&transaction_signature_str, "devnet");
    println!("✅ Success! Mint Token Transaction: {}", link);

    Ok(())
}