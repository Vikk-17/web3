- SPL Token Account
- Borsh Deserailization
- Bytemuck


- Tokens are digital assets that represent ownership over diverse categories of assets. Tokenization enables the digitalization of property rights. Tokens on Solana are referred to as SPL (Solana Program Library) Tokens.

- *Token Programs* contain all instruction logic for interacting with tokens on the network (both fungible and non-fungible).
- A *Mint Account* represents a specific token and stores global metadata about the token such as the total supply and mint authority (address authorized to create new units of a token).
- A *Token Account* tracks individual ownership of tokens for a specific mint account for a specific owner.
- An *Associated Token Account* is a Token Account created with an address derived from the owner and mint account addresses.
