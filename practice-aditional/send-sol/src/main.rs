use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer, Signature},
    system_instruction,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta};
use dotenv::dotenv;
use std::{env, str::FromStr, error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format!");
    let sender = Keypair::from_bytes(&as_array).expect("Failed to create keypair from secret key");

    let connection = RpcClient::new_with_commitment(cluster_api_url("devnet"), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("MAXHwQi54g5uNwLxwHjmv3EVDnRKoV1oTLphZYri2Q9").expect("Invalid recipient public key");
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    let send_sol_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient,
        (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64,
    );

    let memo_text = "My memo text";
    let memo_instruction = Instruction {
        program_id: Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap(),
        accounts: vec![],
        data: memo_text.as_bytes().to_vec(),
    };

    let mut transaction = Transaction::new_with_payer(&[send_sol_instruction, memo_instruction], Some(&sender.pubkey()));

    let recent_blockhash = connection.get_latest_blockhash()?;
    transaction.sign(&[&sender], recent_blockhash);

    let signature = connection.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?;

    println!("✅ Transaction confirmed, signature: {}!", signature);

    // Отримання підтвердженої транзакції з кількома спробами
    let confirmed_transaction = retry_get_transaction(&connection, &signature)?;

    if let Some(logs) = confirmed_transaction
        .transaction
        .meta
        .and_then(|meta| Option::<Vec<String>>::from(meta.log_messages))
    {
        println!("📜 Transaction Logs:");
        for log in logs {
            println!("{}", log);
        }
    } else {
        println!("No logs found for this transaction.");
    }

    Ok(())
}

// Функція для повторного отримання підтвердженої транзакції
fn retry_get_transaction(connection: &RpcClient, signature: &Signature) -> Result<EncodedConfirmedTransactionWithStatusMeta, Box<dyn Error>> {
    for _ in 0..5 {
        match connection.get_transaction(signature, UiTransactionEncoding::Json) {
            Ok(tx) => return Ok(tx),
            Err(e) => {
                eprintln!("Failed to get transaction: {}. Retrying...", e);
                thread::sleep(Duration::from_secs(2));
                continue;
            }
        }
    }
    Err("Failed to get confirmed transaction after multiple attempts.".into())
}

// Функція для отримання URL на Explorer
fn cluster_api_url(cluster: &str) -> &str {
    match cluster {
        "devnet" => "https://api.devnet.solana.com",
        _ => panic!("Unsupported cluster"),
    }
}
