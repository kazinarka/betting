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

pub fn new_game(matches: &ArgMatches) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    let url = match matches.value_of("env") {
        Some("dev") => "https://api.devnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let (betting_pda, _) = Pubkey::find_program_address(&["betting".as_bytes()], &program_id);

    println!("Betting {:?}", betting_pda);

    let value = matches.value_of("value").unwrap().parse::<u64>().unwrap();

    let (supported_token_data, _) = Pubkey::find_program_address(
        &[
            "whitelist".as_bytes(),
            &"8hp71urEffeQFo49wSbe43rwAnj2Mw5sgCDWhWGTzYH1"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    let (user_data, _) = Pubkey::find_program_address(
        &[
            "user".as_bytes(),
            &"BYX8A4T46wfMbyVKty3z8diuLmJydPDrNzwMwKMFz87P"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    let (game_data, _) = Pubkey::find_program_address(
        &[
            "game".as_bytes(),
            &"BYX8A4T46wfMbyVKty3z8diuLmJydPDrNzwMwKMFz87P"
                .parse::<Pubkey>()
                .unwrap()
                .to_bytes(),
        ],
        &program_id,
    );

    println!("Whitelist {:?}", supported_token_data);

    println!("User {:?}", user_data);

    println!("Game {:?}", game_data);

    let source = spl_associated_token_account::get_associated_token_address(
        &wallet_pubkey,
        &"8hp71urEffeQFo49wSbe43rwAnj2Mw5sgCDWhWGTzYH1"
            .parse::<Pubkey>()
            .unwrap(),
    );

    let destination = spl_associated_token_account::get_associated_token_address(
        &game_data,
        &"8hp71urEffeQFo49wSbe43rwAnj2Mw5sgCDWhWGTzYH1"
            .parse::<Pubkey>()
            .unwrap(),
    );

    println!("Source {:?}", source);

    println!("Destination {:?}", destination);

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &BettingInstruction::NewGame {
            value,
            support_bot: false,
        },
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(betting_pda, false),
            AccountMeta::new_readonly(RENT.parse::<Pubkey>().unwrap(), false),
            AccountMeta::new(supported_token_data, false),
            AccountMeta::new(user_data, false),
            AccountMeta::new(game_data, false),
            AccountMeta::new_readonly(
                "HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new_readonly(
                "HgTtcbcmp5BeThax5AU8vg4VwK79qAvAKKFMs8txMLW6"
                    .parse::<Pubkey>()
                    .unwrap(),
                false,
            ),
            AccountMeta::new(source, false),
            AccountMeta::new(destination, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(
                "8hp71urEffeQFo49wSbe43rwAnj2Mw5sgCDWhWGTzYH1"
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
        ],
    )];
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
