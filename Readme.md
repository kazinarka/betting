# Set up + Testing (step by step)
## NOTE: all generating accounts steps only for devnet

- program/Cargo.toml

change package and lib names to "betting-contract" (or another)

- change cluster to devnet

> solana config set --url https://api.devnet.solana.com

- generate admin wallet

> solana-keygen new --outfile id.json

> solana-keygen pubkey id.json

- fund admin address 10 SOL

> solana address -k id.json

5 times

> solana airdrop 2 [admin]

- program/src/consts.rs

change ADMIN const with pubkey of your admin address

- build contract

> make build

- Get pubkey of contract

> solana address -k /betting/target/deploy/betting-contract.json

- program/src/lib.rs

Fill declare_id macro with your pubkey

- client/src/consts.rs

Fill PROGRAM_ID const with your pubkey

- deploy contract

> solana program deploy /betting/target/deploy/betting-contract.so

## Contract deployed!

- generate token

NOTE: Only have to do this in development. In production, the reward token should already exist and you just have to transfer some into the vault later.

`spl-token create-token --decimals 0`

NOTE: Any decimal spl token will work. Just using 0 for development purposes.

`spl-token create-account <mint>`

NOTE: replace <mint> with the returned mint address from above

`spl-token mint <mint> <amount>`

- Makefile

change init

> init:
> 
>cd client; cargo run -- init -e dev -s [your path to admin] -m [manager] -t [token mint]

NOTE: let manager be an admin account (only for development)

- Init platform

> make init

that will create pda with default fields, add manager and add first supported token

- Change close delay

NOTE: only for development, it will reduce close delay from 300 sec to 10 sec

> make change_close_delay

- NOTE: this and others commands you need to modify in Makefile and client/transactions/[transaction] due to your data

- generate user wallet

> solana-keygen new --outfile user.json

> solana-keygen pubkey user.json

- fund user address some SOL

> solana address -k user.json

> solana airdrop [value] [user]

- NOTE: lets use admin as another user to simplify testing

- generate admin-referrer wallet

> solana-keygen new --outfile admin-referrer.json

> solana-keygen pubkey admin-referrer.json

- fund admin-referrer address some SOL

> solana address -k admin-referrer.json

> solana airdrop [value] [admin-referrer]

- generate user-referrer wallet

> solana-keygen new --outfile user-referrer.json

> solana-keygen pubkey user-referrer.json

- fund user-referrer address some SOL

> solana address -k user-referrer.json

> solana airdrop [value] [user-referrer]

- transfer our tokens to users

> spl-token transfer <token_mint> <amount> <user-address> --fund-recipient

- registrate users

> make registration

- create new_game

> make new_game

- forced close the game

> make forced_close

- create new_game

> make new_game

- manually close the game

> make manually_close

- create new_game

> make new_game

- join game

> make join_game

- close game

> make close_game

## Tests completed!

# Mainnet

- program/Cargo.toml

change package and lib names to "betting-contract" (or another)

- change cluster to mainnet

> solana config set --url https://api.mainnet-beta.solana.com

- program/src/consts.rs

change ADMIN const with pubkey of your admin address

- build contract

> make build

- Get pubkey of contract

> solana address -k /betting/target/deploy/betting-contract.json

- program/src/lib.rs

Fill declare_id macro with your pubkey

- client/src/consts.rs

Fill PROGRAM_ID const with your pubkey

- deploy contract

> solana program deploy /betting/target/deploy/betting-contract.so

- init

> make init

## Thats all!