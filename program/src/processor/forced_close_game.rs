use crate::consts::{BETTING, GAME, USER, WHITELIST};
use crate::error::ContractError;
use crate::processor::require;
use crate::state::helpers::{get_betting_info, get_game_info, get_user_info};
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn forced_close(accounts: &[AccountInfo], program_id: &Pubkey, user: Pubkey) -> ProgramResult {
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

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if &betting_info.manager != accounts.payer.key {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (game_pda, game_bump) = Pubkey::find_program_address(&[GAME, &user.to_bytes()], program_id);

    if *accounts.game.key != game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut game_info = get_game_info(&accounts.game.data.borrow())?;

    require(!game_info.closed, "Game already closed")?;
    require(
        game_info.gamer2 == Pubkey::default(),
        "Game started already",
    )?;

    game_info.closed = true;
    game_info.serialize(&mut &mut accounts.game.data.borrow_mut()[..])?;

    let (user_pda, _) = Pubkey::find_program_address(&[USER, &user.to_bytes()], program_id);

    if *accounts.user.key != user_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut user_info = get_user_info(&accounts.user.data.borrow())?;

    user_info.in_game = false;
    user_info.serialize(&mut &mut accounts.user.data.borrow_mut()[..])?;

    let (token_pda, _) = Pubkey::find_program_address(
        &[WHITELIST, &accounts.supported_token.key.to_bytes()],
        program_id,
    );

    if *accounts.supported_token.key != token_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&game_pda, accounts.token.key)
        != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&user, accounts.token.key)
        != accounts.destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.destination.owner != accounts.token_program.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                accounts.payer.key,
                &user,
                accounts.token.key,
            ),
            &[
                accounts.payer.clone(),
                accounts.destination.clone(),
                accounts.user_wallet.clone(),
                accounts.token.clone(),
                accounts.system_program.clone(),
                accounts.token_program.clone(),
                accounts.rent_info.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            accounts.token_program.key,
            accounts.source.key,
            accounts.destination.key,
            accounts.game.key,
            &[],
            game_info.amount1,
        )?,
        &[
            accounts.source.clone(),
            accounts.destination.clone(),
            accounts.game.clone(),
            accounts.token_program.clone(),
        ],
        &[&[GAME, &user.to_bytes(), &[game_bump]]],
    )?;

    invoke_signed(
        &spl_token::instruction::close_account(
            accounts.token_program.key,
            accounts.source.key,
            &user,
            accounts.game.key,
            &[],
        )?,
        &[
            accounts.source.clone(),
            accounts.user_wallet.clone(),
            accounts.game.clone(),
            accounts.token_program.clone(),
        ],
        &[&[GAME, &user.to_bytes(), &[game_bump]]],
    )?;

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
    pub user_wallet: &'a AccountInfo<'b>,
    pub game: &'a AccountInfo<'b>,
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
            user_wallet: next_account_info(acc_iter)?,
            game: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            destination: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            token: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
        })
    }
}
