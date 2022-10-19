use crate::consts::{BETTING, GAME, USER, WHITELIST};
use crate::error::ContractError;
use crate::processor::require;
use crate::state::helpers::{
    get_betting_info, get_game_info, get_supported_token_info, get_user_info,
};
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

pub fn bet_with_join(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    user_master: Pubkey,
    value: u64,
    support_bot: bool,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (token_pda, _) =
        Pubkey::find_program_address(&[WHITELIST, &accounts.token.key.to_bytes()], program_id);

    if *accounts.supported_token.key != token_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (user_pda, _) =
        Pubkey::find_program_address(&[USER, &accounts.payer.key.to_bytes()], program_id);

    if *accounts.user.key != user_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (user_master_pda, _) =
        Pubkey::find_program_address(&[USER, &user_master.to_bytes()], program_id);

    if *accounts.user_master.key != user_master_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (game_pda, _) =
        Pubkey::find_program_address(&[GAME, &accounts.payer.key.to_bytes()], program_id);

    if *accounts.game.key != game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut user_info = get_user_info(&accounts.user.data.borrow())?;
    let user_master_info = get_user_info(&accounts.user_master.data.borrow())?;

    require(
        (&user_info.address == accounts.payer.key) || (user_info.is_bot == true),
        "register first",
    )?;
    require(
        (user_info.in_game == false) || (user_info.is_bot == true),
        "already in game",
    )?;

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    require(betting_info.accept_bets, "bets locked")?;

    let supported_token_info = get_supported_token_info(&accounts.supported_token.data.borrow())?;

    require(
        supported_token_info.mint == *accounts.token.key,
        "Token is not supported",
    )?;

    user_info.support_bots = support_bot;
    user_info.in_game = true;
    user_info.serialize(&mut &mut accounts.user.data.borrow_mut()[..])?;

    let game_info = get_game_info(&accounts.pda.data.borrow())?;

    if (game_info.gamer2 == Pubkey::default()) && !game_info.closed {
        if !support_bot {
            require(!user_master_info.is_bot, "User doesn't support bots")?;
        }
        if !user_master_info.support_bots {
            require(!user_info.is_bot, "User doesn't support bots")?;
        }

        let convert_value = chainlink_solana::latest_round_data(
            accounts.chainlink_program.clone(),
            accounts.feed_account.clone(),
        )?;

        if &spl_associated_token_account::get_associated_token_address(
            accounts.payer.key,
            accounts.token.key,
        ) != accounts.source.key
        {
            return Err(ContractError::InvalidInstructionData.into());
        }

        if &spl_associated_token_account::get_associated_token_address(
            &game_pda,
            accounts.token.key,
        ) != accounts.destination.key
        {
            return Err(ContractError::InvalidInstructionData.into());
        }

        if accounts.destination.owner != accounts.token_program.key {
            invoke(
                &spl_associated_token_account::create_associated_token_account(
                    accounts.payer.key,
                    accounts.game.key,
                    accounts.token.key,
                ),
                &[
                    accounts.payer.clone(),
                    accounts.destination.clone(),
                    accounts.game.clone(),
                    accounts.token.clone(),
                    accounts.system_program.clone(),
                    accounts.token_program.clone(),
                    accounts.rent_info.clone(),
                    accounts.token_assoc.clone(),
                ],
            )?;
        }

        invoke(
            &spl_token::instruction::transfer(
                accounts.token_program.key,
                accounts.source.key,
                accounts.destination.key,
                accounts.payer.key,
                &[],
                value,
            )?,
            &[
                accounts.source.clone(),
                accounts.destination.clone(),
                accounts.payer.clone(),
                accounts.token_program.clone(),
            ],
        )?;

        join_game(
            accounts,
            program_id,
            user_master,
            value,
            convert_value.answer,
        )?;
    }

    Ok(())
}

pub fn join_game(
    accounts: Accounts,
    program_id: &Pubkey,
    user_master: Pubkey,
    amount: u64,
    convert_amount: i128,
) -> ProgramResult {
    let clock = Clock::get()?;

    let (game_pda, _) =
        Pubkey::find_program_address(&[GAME, &accounts.user_master.key.to_bytes()], program_id);

    if accounts.pda.key != &game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut game_info = get_game_info(&accounts.game.data.borrow())?;

    require(*accounts.payer.key != user_master, "double registration")?;

    game_info.gamer2 = *accounts.payer.key;
    game_info.token2 = *accounts.token.key;
    game_info.amount2 = amount;
    game_info.convert_amount2 = convert_amount;
    game_info.latest_bet = clock.unix_timestamp as u64;
    game_info.serialize(&mut &mut accounts.game.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub supported_token: &'a AccountInfo<'b>,
    pub user: &'a AccountInfo<'b>,
    pub user_master: &'a AccountInfo<'b>,
    pub game: &'a AccountInfo<'b>,
    pub chainlink_program: &'a AccountInfo<'b>,
    pub feed_account: &'a AccountInfo<'b>,
    pub source: &'a AccountInfo<'b>,
    pub destination: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub token: &'a AccountInfo<'b>,
    pub token_assoc: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            pda: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            supported_token: next_account_info(acc_iter)?,
            user: next_account_info(acc_iter)?,
            user_master: next_account_info(acc_iter)?,
            game: next_account_info(acc_iter)?,
            chainlink_program: next_account_info(acc_iter)?,
            feed_account: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            destination: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            token: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
        })
    }
}
