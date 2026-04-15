mod model;

use solana_sdk::{
    pubkey, // macro, generates staic public key
    pubkey::Pubkey,
    signer::{
        keypair::Keypair, // vanilla Ed25519 keypair
        Signer, // -> import this to use pubkey function
    }
};
// use model::Account;


#[tokio::main]
async fn main() {
    let keypair = Keypair::new(); // random Keypair using OsRang
    println!("Public key: {:?}", keypair.pubkey());
    println!("Private key: {:?}", keypair.to_bytes());
    println!("Base58 encoded string public key: {:?}", keypair.to_base58_string());


    let program_address = pubkey!("11111111111111111111111111111111"); // 32 1's
    println!("{:?}", program_address);
    let seeds = [b"testing ".as_ref()]; // coming from ascii table t=116
    println!("{:?}", seeds);
    // seeds: &[&[u8]], program_address: &Address
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
}
