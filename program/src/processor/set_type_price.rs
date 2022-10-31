use crate::consts::{BETTING, TYPE_PRICE};
use crate::error::ContractError;
use crate::state::helpers::get_betting_info;
use crate::state::structs::TypePrice;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn set_type_price(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    t: u64,
    price: u64,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if *accounts.payer.key != betting_info.manager || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (data_address, data_address_bump) =
        Pubkey::find_program_address(&[TYPE_PRICE, t.to_string().as_bytes()], program_id);

    if *accounts.type_price.key != data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.type_price.owner != program_id {
        let size: u64 = 8;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.type_price.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &data_address, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.type_price.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&data_address, size),
            &[accounts.type_price.clone(), accounts.system_program.clone()],
            &[&[TYPE_PRICE, t.to_string().as_bytes(), &[data_address_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&data_address, program_id),
            &[accounts.type_price.clone(), accounts.system_program.clone()],
            &[&[TYPE_PRICE, t.to_string().as_bytes(), &[data_address_bump]]],
        )?;
    }

    let type_price = TypePrice { price };
    type_price.serialize(&mut &mut accounts.type_price.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub type_price: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            pda: next_account_info(acc_iter)?,
            type_price: next_account_info(acc_iter)?,
        })
    }
}
