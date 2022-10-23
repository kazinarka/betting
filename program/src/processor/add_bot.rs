use crate::consts::{ADMIN, USER};
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

pub fn add_bot(accounts: &[AccountInfo], program_id: &Pubkey, bot: Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (data_address, data_address_bump) =
        Pubkey::find_program_address(&[USER, &bot.to_bytes()], program_id);

    if *accounts.bot.key != data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.bot.owner != program_id {
        let size: u64 = 32 + 32 + 1 + 1 + 1 + 8;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.bot.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &data_address, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.bot.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&data_address, size),
            &[accounts.bot.clone(), accounts.system_program.clone()],
            &[&[USER, &bot.to_bytes(), &[data_address_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&data_address, program_id),
            &[accounts.bot.clone(), accounts.system_program.clone()],
            &[&[USER, &bot.to_bytes(), &[data_address_bump]]],
        )?;
    }

    let user = User {
        address: bot,
        referrer: Pubkey::default(),
        in_game: false,
        support_bots: false,
        is_bot: true,
        turnover: 0,
    };
    user.serialize(&mut &mut accounts.bot.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub bot: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            bot: next_account_info(acc_iter)?,
        })
    }
}
