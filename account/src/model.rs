use solana_sdk::pubkey::Pubkey;
use solana_sdk;

pub struct Account {
    /// total lamports in this account, has to hold minimum to be on chain
    /// 1 SOL = 1,000,000,000 lamports
    pub lamports: u64,
    /// data held in this account
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub owner: Pubkey,
    /// If true, this account's data contains a program (and is now read-only)
    pub executable: bool,
    // deprecated
    // pub rent_epoch: Epoch,
}
