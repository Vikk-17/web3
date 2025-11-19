## Concepts

- Accounts
    - Address - A 32bytes unique id to locate the account on the solana network
    - Generate Public Key using solana_sdk
    - Pda
    - lamports: The account balance is lamport. Every account must have a minimum lamport balance, called rent, which allows its data to be stored on-chain. Rent is proportional to the size of the account.



### Account structure

- Every Account has a max size of 10MiB (10 Mebibyte)
    - lamports: The number of lamports in the account
    - data: The account's data
    - owner: The ID of the program that owns the program
    - executable: Indicates whether the account contains executable binary
    - rent_epoch: The deprecated rent epoch field

```
1MiB = 2^20 bytes
10MiB = 10 * 1MiB = 10 * 1024 * 1024
```

