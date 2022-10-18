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
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

pub fn bet_with_join(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    gamer: Pubkey,
    user_master: Pubkey,
    token: Pubkey,
    _value: u64,
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

    require(user_info.referrer == gamer, "register first")?;
    require(user_info.in_game == false, "already in game")?;

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    require(betting_info.accept_bets, "bets locked")?;

    let supported_token_info = get_supported_token_info(&accounts.supported_token.data.borrow())?;

    require(supported_token_info.mint == token, "Token is not supported")?;

    let game_info = get_game_info(&accounts.game.data.borrow())?;

    let (user_master_pda, _) =
        Pubkey::find_program_address(&[USER, &user_master.to_bytes()], program_id);

    if *accounts.user_master.key != user_master_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let user_master_info = get_user_info(&accounts.user_master.data.borrow())?;

    if !game_info.closed && (game_info.gamer2 == Pubkey::default()) {
        if !support_bot {
            require(!user_master_info.support_bots, "User doesn't support bots")?;
        }
    }

    user_info.support_bots = support_bot;
    user_info.in_game = true;

    join_game(accounts, program_id, gamer, token, Pubkey::default(), 0)?;

    Ok(())
}

pub fn join_game(
    accounts: Accounts,
    program_id: &Pubkey,
    creator: Pubkey,
    user: Pubkey,
    token: Pubkey,
    amount: u64,
) -> ProgramResult {
    let clock = Clock::get()?;

    let (game_pda, _) = Pubkey::find_program_address(&[GAME, &creator.to_bytes()], program_id);

    if accounts.pda.key != &game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut game_info = get_game_info(&accounts.pda.data.borrow())?;

    require(user != creator, "double registration")?;

    game_info.gamer2 = user;
    game_info.token2 = token;
    game_info.amount2 = amount;
    game_info.latest_bet = clock.unix_timestamp as u64;
    game_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

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
            user_master: next_account_info(acc_iter)?,
            game: next_account_info(acc_iter)?,
            bot: next_account_info(acc_iter)?,
        })
    }
}
