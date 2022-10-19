use crate::consts::{ADMIN, BETTING};
use crate::error::ContractError;
use crate::state::helpers::get_betting_info;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn change_close_delay(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    new_delay: u64,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    betting_info.close_delay = new_delay;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn lock_bets(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    betting_info.accept_bets = false;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn unlock_bets(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    betting_info.accept_bets = true;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn new_manager(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    manager: Pubkey,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    betting_info.manager = manager;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn set_global_fee(accounts: &[AccountInfo], program_id: &Pubkey, fee: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if *accounts.payer.key != betting_info.manager || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    betting_info.global_fee = fee;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn set_admin_fee(accounts: &[AccountInfo], program_id: &Pubkey, fee: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if *accounts.payer.key != betting_info.manager || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    betting_info.admin_fee = fee;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn set_winner_fee(accounts: &[AccountInfo], program_id: &Pubkey, fee: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if *accounts.payer.key != betting_info.manager || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    betting_info.referrer_fee = fee;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn set_transaction_fee(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    fee: u64,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if *accounts.payer.key != betting_info.manager || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    betting_info.transaction_fee = fee;
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            pda: next_account_info(acc_iter)?,
        })
    }
}
