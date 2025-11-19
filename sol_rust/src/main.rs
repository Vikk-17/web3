// use solana_sdk::signer::{
//     keypair::Keypair,
//     Signer,
// };
use solana_sdk::pubkey;
// use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // let keypair = Keypair::new();
    // println!("Public key: {}", keypair.pubkey());
    // println!("Secret key: {:?}", keypair.to_bytes());
    //
    // // -----------------------------------------------
    // let program_address = pubkey!("11111111111111111111111111111111");
    // let seeds = [b"chakraborty".as_ref()];
    // let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    //
    // println!("PDA: {pda}");
    // println!("Bump: {bump}");

    // Established the connection
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    let account_info = connection.get_account(&program_id).await?;
    println!("Account info: {:#?}", account_info);

    Ok(())
}
