use crate::consts::{PROGRAM_ID, RENT};
use crate::structs::BettingInstruction;
use clap::ArgMatches;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;

pub fn close_game(matches: &ArgMatches) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    let url = match matches.value_of("env") {
        Some("dev") => "https://api.testnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let (betting_pda, _) = Pubkey::find_program_address(&["betting".as_bytes()], &program_id);

    println!("Betting {:?}", betting_pda);

    let user = matches.value_of("user").unwrap().parse::<Pubkey>().unwrap();

    let winner = matches
        .value_of("winner")
        .unwrap()
        .parse::<Pubkey>()
        .unwrap();

    let t = matches.value_of("type").unwrap().parse::<u64>().unwrap();

    let (supported_token_data, _) = Pubkey::find_program_address(
        &[
            "whitelist".as_bytes(),
            &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    let (user_data, _) = Pubkey::find_program_address(
        &[
            "user".as_bytes(),
            &"4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    let (user1_data, _) = Pubkey::find_program_address(
        &[
            "user".as_bytes(),
            &"9LZr77sE8J6bHYXcZXM9AeUJEssWZKh3AhmaXj3G7uUn"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    let (game_data, _) = Pubkey::find_program_address(
        &[
            "game".as_bytes(),
            &"4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    println!("Whitelist {:?}", supported_token_data);

    println!("User {:?}", user_data);

    println!("User1 {:?}", user1_data);

    println!("Game {:?}", game_data);

    let source = spl_associated_token_account::get_associated_token_address(
        &game_data,
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    println!("Source {:?}", source);

    let destination_user = spl_associated_token_account::get_associated_token_address(
        &"4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
            .parse::<Pubkey>()
            .unwrap(),
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    let destination_user1 = spl_associated_token_account::get_associated_token_address(
        &"9LZr77sE8J6bHYXcZXM9AeUJEssWZKh3AhmaXj3G7uUn"
            .parse::<Pubkey>()
            .unwrap(),
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    println!("Destination user 1 {:?}", destination_user);

    println!("Destination user 2 {:?}", destination_user1);

    let destination_user_referrer = spl_associated_token_account::get_associated_token_address(
        &"AvjgZccj5UfPiVVTEb7Zc132k7MVFiKswcB9AtnCqKea"
            .parse::<Pubkey>()
            .unwrap(),
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    let destination_user1_referrer = spl_associated_token_account::get_associated_token_address(
        &"6G7Sc3MjR4AZDAgNJZJmSpLuiNUCRksF3bN8opeX2Fuj"
            .parse::<Pubkey>()
            .unwrap(),
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    println!(
        "Destination user 1 referrer {:?}",
        destination_user_referrer
    );

    println!(
        "Destination user 2 referrer {:?}",
        destination_user1_referrer
    );

    let destination_owner = spl_associated_token_account::get_associated_token_address(
        &"4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
            .parse::<Pubkey>()
            .unwrap(),
        &"Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
            .parse::<Pubkey>()
            .unwrap(),
    );

    println!("Destination owner {:?}", destination_owner);

    let (type_price_pda, _) = Pubkey::find_program_address(
        &["type_price".as_bytes(), t.to_string().as_bytes()],
        &program_id,
    );

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &BettingInstruction::Close {
            user,
            winner_address: winner,
            t,
        },
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(betting_pda, false),
            AccountMeta::new_readonly(RENT.parse::<Pubkey>().unwrap(), false),
            AccountMeta::new(supported_token_data, false),
            AccountMeta::new(supported_token_data, false),
            AccountMeta::new(user_data, false),
            AccountMeta::new(user1_data, false),
            AccountMeta::new(
                "4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(
                "9LZr77sE8J6bHYXcZXM9AeUJEssWZKh3AhmaXj3G7uUn"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(game_data, false),
            AccountMeta::new(source, false),
            AccountMeta::new(source, false),
            AccountMeta::new(destination_user, false),
            AccountMeta::new(destination_user, false),
            AccountMeta::new(destination_user1, false),
            AccountMeta::new(destination_user1, false),
            AccountMeta::new(
                "AvjgZccj5UfPiVVTEb7Zc132k7MVFiKswcB9AtnCqKea"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(
                "6G7Sc3MjR4AZDAgNJZJmSpLuiNUCRksF3bN8opeX2Fuj"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(destination_user_referrer, false),
            AccountMeta::new(destination_user1_referrer, false),
            AccountMeta::new(destination_user_referrer, false),
            AccountMeta::new(destination_user1_referrer, false),
            AccountMeta::new(
                "4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(destination_owner, false),
            AccountMeta::new(destination_owner, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(
                "Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new_readonly(
                "Kg7atGGZGiznRLRfbCizcJvcZdSzjYURRJqwEdx5Xqe"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new_readonly(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(type_price_pda, false),
        ],
    )];
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
