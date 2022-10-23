# Infrastructure

## Program
`cd program`
> program/src
- Source files for staking smart contract program

>program/tests
- Tests for all instructions in devnet cluster and for reward calculation flow

## Client
`cd client`
- Simple service to call SC instructions via command line

# Setup + Commands

## Set ADMIN and REWARD_MINT consts in `program/src/consts.rs` and  enter program id to declare_id macro in `program/src/lib.rs`

`cd program && cargo build-bpf`

- NOTE: If `cargo build-bpf` doesn't work for you, run `rm -rf ~/.cache/solana` and then re-run the build command again. This should force solana to re-download and link the bpf utilities.

## Deployment will cost 1.75046512 sol

`solana program deploy /path/to/nft-staking/program/target/deploy/betting_platform.so`

## Set REWARD_MINT and PROGRAM_ID consts in `client/src/consts.rs`

## Run commands below in `rust-client` directory

`cargo build`

- NOTE: if you want to call devnet contract, just add `-e dev` to commands in command line

## Generate vault and transfer reward tokens into the vault

`cargo run -- generate_vault_address -s /path/to/deployer/id.json`

`spl-token transfer <reward_mint> <amount> <vault-address> --fund-recipient`

- NOTE: second address is the vault address returned from `generate_vault_address` cmd

Betting 95wR32j8XvczRxPhcvjqxeNbDQS1LnTsNZRW4JGyrpcu
Whitelist 2HgPbzn5xAkWEpFzbqX2b7vXzAhRjGxEwvFc4f6iobav
User 3o2hUqpomz64tyc8V5cTCY9XuQ27HqiwuBxHZex8nUQY
Game GjEgSND3m8GAyb2Z6xzadJS1VGdZRqzJcFc5Vo19JQh6
Source 43dSjwKNjEsFo9Q5xN9gZ4D3gW3SUrdT184qMaL76oSf
Destination HwpVuoTf57SwPHyjSit1SZjCjufhcQqMUK9Bwx95vDRf


92at8vLn35rRZqAQ8LGR6Whmtc2b1c2H8vChnNrziY6w