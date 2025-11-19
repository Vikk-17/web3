use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction
};
// use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::{
    id as token_2022_program_id,
    instruction::initialize_mint,
    state::Mint,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create connection to the local validator
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );
    let recent_blockhash = client.get_latest_blockhash().await?;

    // Generate a new keypair for the fee payer
    let fee_payer = Keypair::new();

    // Airdrop 1 SOL to fee payer
    let airdrop_signature = client
        .request_airdrop(&fee_payer.pubkey(), 1_000_000_000)
        .await?;

    loop {
        let confirmed = client
            .confirm_transaction(&airdrop_signature)
            .await?;

        if confirmed {
            break;
        }
    }


    Ok(())
}
