use crate::consts::{BETTING, GAME, PRECISION, TYPE_PRICE, USER, WHITELIST};
use crate::error::ContractError;
use crate::processor::require;
use crate::state::helpers::{
    get_betting_info, get_supported_token_info, get_type_price_info, get_user_info,
};
use crate::state::structs::Game;
use borsh::BorshSerialize;
use chainlink_solana;
use num_traits::ToPrimitive;
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
    t: u64,
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

    let (type_price, _) =
        Pubkey::find_program_address(&[TYPE_PRICE, t.to_string().as_bytes()], program_id);

    if *accounts.type_price.key != type_price {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let type_price_info = get_type_price_info(&accounts.type_price.data.borrow())?;

    let value = type_price_info.price;

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

    let (game_pda, _) =
        Pubkey::find_program_address(&[GAME, &accounts.payer.key.to_bytes()], program_id);

    if *accounts.game.key != game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut user_info = get_user_info(&accounts.user.data.borrow())?;

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

    require(
        supported_token_info.feed == *accounts.feed_account.key,
        "Wrong feed for this token",
    )?;

    user_info.support_bots = support_bot;
    user_info.in_game = true;
    user_info.serialize(&mut &mut accounts.user.data.borrow_mut()[..])?;

    let convert_value: i128 = chainlink_solana::latest_round_data(
        accounts.chainlink_program.clone(),
        accounts.feed_account.clone(),
    )?
    .answer;

    let answer = convert_value.to_u64().unwrap();

    if &spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.token.key,
    ) != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&game_pda, accounts.token.key)
        != accounts.destination.key
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
            value * PRECISION * PRECISION / answer,
        )?,
        &[
            accounts.source.clone(),
            accounts.destination.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
        ],
    )?;

    new_game(
        accounts,
        program_id,
        value * PRECISION * PRECISION / answer,
        value,
    )?;

    Ok(())
}

pub fn new_game(
    accounts: Accounts,
    program_id: &Pubkey,
    amount: u64,
    type_price: u64,
) -> ProgramResult {
    let clock = Clock::get()?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (game_pda, game_bump_seed) =
        Pubkey::find_program_address(&[GAME, &accounts.payer.key.to_bytes()], program_id);

    if accounts.game.key != &game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.game.owner != program_id {
        let size: u64 = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 1 + 8;

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
            &[&[GAME, &accounts.payer.key.to_bytes(), &[game_bump_seed]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&game_pda, program_id),
            &[accounts.game.clone(), accounts.system_program.clone()],
            &[&[GAME, &accounts.payer.key.to_bytes(), &[game_bump_seed]]],
        )?;
    }

    let game_info = Game {
        gamer1: *accounts.payer.key,
        gamer2: Pubkey::default(),
        token1: *accounts.token.key,
        token2: Pubkey::default(),
        amount1: amount,
        amount2: 0,
        latest_bet: clock.unix_timestamp as u64,
        closed: false,
        type_price,
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
    pub chainlink_program: &'a AccountInfo<'b>,
    pub feed_account: &'a AccountInfo<'b>,
    pub source: &'a AccountInfo<'b>,
    pub destination: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub token: &'a AccountInfo<'b>,
    pub token_assoc: &'a AccountInfo<'b>,
    pub type_price: &'a AccountInfo<'b>,
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
            chainlink_program: next_account_info(acc_iter)?,
            feed_account: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            destination: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            token: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
            type_price: next_account_info(acc_iter)?,
        })
    }
}
