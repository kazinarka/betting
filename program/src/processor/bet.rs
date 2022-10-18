use crate::consts::{BETTING, GAME, USER, WHITELIST};
use crate::error::ContractError;
use crate::processor::require;
use crate::state::helpers::{get_betting_info, get_supported_token_info, get_user_info};
use crate::state::structs::Game;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn bet(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    gamer: Pubkey,
    token: Pubkey,
    value: u64,
    support_bot: bool,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (token_pda, _) = Pubkey::find_program_address(&[WHITELIST, &token.to_bytes()], program_id);

    if *accounts.supported_token.key != token_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (user_pda, _) = Pubkey::find_program_address(&[USER, &gamer.to_bytes()], program_id);

    if *accounts.user.key != user_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (game_pda, _) = Pubkey::find_program_address(&[GAME, &gamer.to_bytes()], program_id);

    if *accounts.game.key != game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (bot_pda, _) = Pubkey::find_program_address(&[WHITELIST, &gamer.to_bytes()], program_id);

    if *accounts.bot.key != bot_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut user_info = get_user_info(&accounts.user.data.borrow())?;

    require(
        (user_info.address == gamer) || (user_info.is_bot == true),
        "register first",
    )?;
    require(
        (user_info.in_game == false) || (user_info.is_bot == true),
        "already in game",
    )?;

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    require(betting_info.accept_bets, "bets locked")?;

    let supported_token_info = get_supported_token_info(&accounts.supported_token.data.borrow())?;

    require(supported_token_info.mint == token, "Token is not supported")?;

    user_info.support_bots = support_bot;
    user_info.in_game = true;

    new_game(accounts, program_id, gamer, token, value)?;

    Ok(())
}

pub fn new_game(
    accounts: Accounts,
    program_id: &Pubkey,
    user: Pubkey,
    token: Pubkey,
    amount: u64,
) -> ProgramResult {
    let clock = Clock::get()?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (game_pda, game_bump_seed) =
        Pubkey::find_program_address(&[GAME, &user.to_bytes()], program_id);

    if accounts.game.key != &game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.game.owner != program_id {
        let size: u64 = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 1;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.game.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &game_pda, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.game.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&game_pda, size),
            &[accounts.game.clone(), accounts.system_program.clone()],
            &[&[GAME, &user.to_bytes(), &[game_bump_seed]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&game_pda, program_id),
            &[accounts.game.clone(), accounts.system_program.clone()],
            &[&[GAME, &user.to_bytes(), &[game_bump_seed]]],
        )?;
    }

    let game_info = Game {
        gamer1: user,
        gamer2: Pubkey::default(),
        token1: token,
        token2: Pubkey::default(),
        amount1: amount,
        amount2: 0,
        latest_bet: clock.unix_timestamp as u64,
        closed: false,
    };
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
    pub game: &'a AccountInfo<'b>,
    pub bot: &'a AccountInfo<'b>,
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
            game: next_account_info(acc_iter)?,
            bot: next_account_info(acc_iter)?,
        })
    }
}
