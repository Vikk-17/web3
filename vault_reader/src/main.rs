use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use bytemuck::{Zeroable, Pod};

/// Mint data.
/// packed is used to remove padding between field
/// repr(C) guarantees C-style memory layout
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct MintLayout {
    pub mint_authority_option: u32,     // <- offset 0  (4 bytes)
    pub mint_authority: [u8; 32],       // <- offset 4  (32 bytes)
    pub supply: u64,                    // <- offset 36 (8 bytes)
    pub decimals: u8,                   // <- offset 44 (1 byte)
    pub is_initialized: u8,             // <- offset 45 (1 byte)
    pub freeze_authority_option: u32,   // <- offset 46 (4 bytes)
    pub freeze_authority: [u8; 32],     // <- offset 50 (32 bytes)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed()
    );
    let pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;
    let account = client.get_account(&pubkey).await?;
    let account_info = client.get_account_data(&pubkey).await?;
    println!("{:#?}", account);
    println!("Data size: {}", account.data.len());

    let _mint_map = MintLayout {
        mint_authority_option: u32::from_ne_bytes(account_info[0..4].try_into().unwrap()),
        mint_authority: account_info[4..36].try_into().unwrap(),
        supply: u64::from_ne_bytes(account_info[36..44].try_into().unwrap()),
        decimals: account_info[44],
        is_initialized: account_info[45],
        freeze_authority_option: u32::from_ne_bytes(account_info[46..50].try_into().unwrap()),
        freeze_authority: account_info[50..82].try_into().unwrap(),
    };

    let mint: &MintLayout = bytemuck::from_bytes(&account.data);
    let decimals = mint.decimals;
    let supply = mint.supply;
    let is_initialized = mint.is_initialized;
    println!("Mint Decimals: {}", decimals);
    println!("Supply: {}", supply);
    println!("Init: {}", is_initialized);


    // println!("<--- Printing the Mint --->");
    // println!("{:#?}", mint_map);
    Ok(())
}

