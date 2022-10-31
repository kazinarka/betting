use crate::consts::{ADMIN, BETTING, WHITELIST};
use crate::error::ContractError;
use crate::state::structs::{BettingInfo, SupportedToken};
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn init(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    manager: Pubkey,
    supported_token: Pubkey,
    feed: Pubkey,
    is_stablecoin: bool,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (betting_pda, betting_bump_seed) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if accounts.pda.owner != program_id {
        let size: u64 = 8 + 8 + 8 + 8 + 1 + 8 + 32;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.pda.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &betting_pda, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.pda.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&betting_pda, size),
            &[accounts.pda.clone(), accounts.system_program.clone()],
            &[&[BETTING, &[betting_bump_seed]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&betting_pda, program_id),
            &[accounts.pda.clone(), accounts.system_program.clone()],
            &[&[BETTING, &[betting_bump_seed]]],
        )?;
    }

    let betting_info = BettingInfo {
        referrer_fee: 50,
        admin_fee: 50,
        global_fee: 10,
        transaction_fee: 0,
        accept_bets: true,
        close_delay: 300,
        manager,
    };
    betting_info.serialize(&mut &mut accounts.pda.data.borrow_mut()[..])?;

    let (data_address, data_address_bump) =
        Pubkey::find_program_address(&[WHITELIST, &supported_token.to_bytes()], program_id);

    if *accounts.supported_token.key != data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.supported_token.owner != program_id {
        let size: u64 = 32 + 32 + 1;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.supported_token.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &data_address, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.supported_token.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&data_address, size),
            &[
                accounts.supported_token.clone(),
                accounts.system_program.clone(),
            ],
            &[&[WHITELIST, &supported_token.to_bytes(), &[data_address_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&data_address, program_id),
            &[
                accounts.supported_token.clone(),
                accounts.system_program.clone(),
            ],
            &[&[WHITELIST, &supported_token.to_bytes(), &[data_address_bump]]],
        )?;
    }

    let supported_token = SupportedToken {
        mint: supported_token,
        feed,
        is_stablecoin,
    };
    supported_token.serialize(&mut &mut accounts.supported_token.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub supported_token: &'a AccountInfo<'b>,
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
        })
    }
}
