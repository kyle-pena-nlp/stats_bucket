use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{str::FromStr, sync::Arc};

fn main() {
    // Initialize client and program ID
    let client = RpcClient::new("https://api.devnet.solana.com");
    let program_id = Pubkey::from_str("4CEx9rGKcdippjxqnamw2rCY4pzxXMCoq3EiJUz53qG9").unwrap();

    let keypair_data: &str = include_str!("local.keypair");
    let keypair_bytes: Vec<u8> = serde_json::from_str(keypair_data).expect("Failed to parse keypair");
    let payer = Keypair::from_bytes(&keypair_bytes).expect("Failed to create Keypair");

    // Define the instruction data (e.g., 5)
    let instruction_data = vec![5u8];

    // Create instruction
    let instruction = Instruction::new_with_bytes(program_id, &instruction_data, vec![]);
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

    // Send transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);
    client.send_and_confirm_transaction(&transaction).unwrap();

    println!("Transaction sent!");
}