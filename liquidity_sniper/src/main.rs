use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use bytemuck::{Zeroable, Pod};

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct RaydiumPool {
    pub _padding_a: [u128; 30],
    pub _padding_b: [u128; 5],
    pub base_vault: [u8; 32],
    pub quote_vault: [u8; 32],
    pub _padding_c: [u128; 8],
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    );
    let pubkey = Pubkey::from_str("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")?;
    let account = client.get_account(&pubkey).await?;
    // println!("Account: {:#?}", account);
    println!("Data length: {}", account.data.len());

    let raydium_pool: &RaydiumPool = bytemuck::from_bytes(&account.data);
    let base_vault = Pubkey::new_from_array(raydium_pool.base_vault);
    let quote_vault = Pubkey::new_from_array(raydium_pool.quote_vault);
    println!("Base vault: {:#?}", base_vault);
    println!("Quote vault: {:#?}", quote_vault);
    Ok(())
}
