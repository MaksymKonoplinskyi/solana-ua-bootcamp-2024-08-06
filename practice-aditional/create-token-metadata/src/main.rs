use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use mpl_token_metadata::instruction::create_metadata_account_v3; // Спробуємо цей варіант
use mpl_token_metadata::state::DataV2; // Зміна на модуль `state` для даних
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");

    let as_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let user = Keypair::from_bytes(&as_vec).expect("Failed to create Keypair");

    // Підключення до Solana Devnet
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let token_mint_account = Pubkey::from_str("6KEmjrDXwez9azPNL3t4XP7PieZyhsG8WnvF6ihgRswb").unwrap();

    let metadata_data = DataV2 {
        name: "Solana UA Bootcamp 2024-08-06".to_string(),
        symbol: "UAB-2".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // Генерація PDA для метаданих
    let (metadata_pda, _metadata_bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
            &token_mint_account.to_bytes(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    // Створюємо інструкцію для створення облікового запису метаданих
    let create_metadata_instruction = create_metadata_account_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_pda,
        token_mint_account,
        user.pubkey(),
        user.pubkey(),
        user.pubkey(),
        metadata_data,
        None,
        None,
        None,
    );

    // Створюємо транзакцію
    let mut transaction = Transaction::new_with_payer(
        &[create_metadata_instruction],
        Some(&user.pubkey()),
    );

    // Отримуємо останній блок для підпису
    let recent_blockhash = connection.get_latest_blockhash().unwrap();
    transaction.sign(&[&user], recent_blockhash);

    // Відправляємо транзакцію
    let signature = connection.send_and_confirm_transaction(&transaction).unwrap();

    // Генеруємо посилання на токен у Solana Explorer
    let token_mint_link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_mint_account
    );
    println!("✅ Look at the token mint again: {}!", token_mint_link);
}
