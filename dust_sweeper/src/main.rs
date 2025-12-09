use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, transaction::Transaction
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_system_interface::instruction::transfer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // create the client -> to local test validator
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );

    // Gnerate burner wallets
    let burner_address: [Keypair; 3] = [Keypair::new(), Keypair::new(), Keypair::new()];

    // Generate master wallet
    let master_keypair = Keypair::new();
    let master_public_key = master_keypair.pubkey();

    // Get balance master wallet
    let balance_master = client.get_balance(&master_public_key).await?;
    println!("Balance of master wallet before transaction: {:#?}",
        balance_master as f64 / LAMPORTS_PER_SOL as f64);

    for (i, burner)  in burner_address.iter().enumerate() {
        let burner_pubkey = burner.pubkey();

        // airdrop to the wallets -> fund wallets
        let txn_signature = client
            .request_airdrop(&burner_pubkey, 10_000_000)
            .await?;

        loop {
            if client.confirm_transaction(&txn_signature).await? {
                break;
            }
        }

        let balance = client.get_balance(&burner_pubkey).await?;
        println!("[Burner {}] balance: {:#?} SOL", i+1, balance as f64 / LAMPORTS_PER_SOL as f64);

        // Send sols to master wallet
        // Create and send transfer transaction
        let transfer_ix = transfer(
            &burner_pubkey,
            &master_public_key,
            balance.saturating_sub(5000),
        );

        let latest_blockhash = client.get_latest_blockhash().await?;
        let txn = Transaction::new_signed_with_payer(&[transfer_ix],
            Some(&burner_pubkey),
            &[burner],
            latest_blockhash
        );

        client.send_and_confirm_transaction(&txn).await?;
        println!("[Burner {}] Swept! Remaining: 0 SOL", i + 1);
    }

    let balance_master = client.get_balance(&master_public_key).await?;
    println!("Master wallet Final Balance (after txn): {:#?} SOL", balance_master as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}
