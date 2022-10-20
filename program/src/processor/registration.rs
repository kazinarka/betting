use crate::consts::USER;
use crate::error::ContractError;
use crate::state::structs::User;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use crate::processor::require;

pub fn registration(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    referrer: Pubkey,
    password: String,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (data_address, data_address_bump) =
        Pubkey::find_program_address(&[USER, &accounts.payer.key.to_bytes()], program_id);

    if *accounts.user.key != data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    require(&referrer != accounts.payer.key, "refferer must not be equal to user wallet")?;

    let password = password.into_bytes();
    let len = password.len() as u64;

    if accounts.user.owner != program_id {
        let size: u64 = 32 + 32 + (len * 2) + 1 + 1 + 1 + 8;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.user.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &data_address, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.user.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&data_address, size),
            &[accounts.user.clone(), accounts.system_program.clone()],
            &[&[USER, &accounts.payer.key.to_bytes(), &[data_address_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&data_address, program_id),
            &[accounts.user.clone(), accounts.system_program.clone()],
            &[&[USER, &accounts.payer.key.to_bytes(), &[data_address_bump]]],
        )?;
    }

    let user = User {
        address: *accounts.payer.key,
        referrer,
        password,
        in_game: false,
        support_bots: false,
        is_bot: false,
        turnover: 0,
    };
    user.serialize(&mut &mut accounts.user.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub user: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            user: next_account_info(acc_iter)?,
        })
    }
}
