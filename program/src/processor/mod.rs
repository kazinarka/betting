pub mod add_bot;
pub mod add_supported_token;
pub mod bet;
pub mod close_game;
pub mod forced_close_game;
pub mod init;
pub mod join_game;
pub mod manually_close_game;
pub mod registration;
pub mod set_type_price;
pub mod setters;

use crate::error::ContractError;
use crate::instruction::BettingInstruction;
use crate::processor::add_bot::add_bot;
use crate::processor::add_supported_token::add_supported_token;
use crate::processor::bet::bet;
use crate::processor::close_game::close;
use crate::processor::forced_close_game::forced_close;
use crate::processor::init::init;
use crate::processor::join_game::bet_with_join;
use crate::processor::manually_close_game::manually_close;
use crate::processor::registration::registration;
use crate::processor::set_type_price::set_type_price;
use crate::processor::setters::{
    change_close_delay, lock_bets, new_manager, set_admin_fee, set_global_fee, set_transaction_fee,
    set_winner_fee, unlock_bets,
};
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use solana_program::pubkey::Pubkey;
use spl_token::error::TokenError;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: BettingInstruction =
            match BettingInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            BettingInstruction::Init {
                manager,
                supported_token,
                feed,
                is_stablecoin,
            } => init(
                accounts,
                program_id,
                manager,
                supported_token,
                feed,
                is_stablecoin,
            )?,
            BettingInstruction::ChangeCloseDelay { new_delay } => {
                change_close_delay(accounts, program_id, new_delay)?
            }
            BettingInstruction::LockBets => lock_bets(accounts, program_id)?,
            BettingInstruction::UnlockBets => unlock_bets(accounts, program_id)?,
            BettingInstruction::AddSupportedToken {
                supported_token,
                feed,
                is_stablecoin,
            } => add_supported_token(accounts, program_id, supported_token, feed, is_stablecoin)?,
            BettingInstruction::Registration { referrer, password } => {
                registration(accounts, program_id, referrer, password)?
            }
            BettingInstruction::NewManager { manager } => {
                new_manager(accounts, program_id, manager)?
            }
            BettingInstruction::SetGlobalFee { fee } => set_global_fee(accounts, program_id, fee)?,
            BettingInstruction::SetAdminFee { fee } => set_admin_fee(accounts, program_id, fee)?,
            BettingInstruction::SetWinnerFee { fee } => set_winner_fee(accounts, program_id, fee)?,
            BettingInstruction::SetTransactionFee { fee } => {
                set_transaction_fee(accounts, program_id, fee)?
            }
            BettingInstruction::AddBot { bot } => add_bot(accounts, program_id, bot)?,
            BettingInstruction::NewGame { t, support_bot } => {
                bet(accounts, program_id, t, support_bot)?
            }
            BettingInstruction::JoinGame {
                t,
                support_bot,
                user_master,
            } => bet_with_join(accounts, program_id, user_master, t, support_bot)?,
            BettingInstruction::ForcedClose { user } => forced_close(accounts, program_id, user)?,
            BettingInstruction::ManuallyClose => manually_close(accounts, program_id)?,
            BettingInstruction::Close {
                user,
                winner_address,
                t,
            } => close(accounts, program_id, user, winner_address, t)?,
            BettingInstruction::SetTypePrice { t, price } => {
                set_type_price(accounts, program_id, t, price)?
            }
        };

        Ok(())
    }
}

pub fn require<E: PrintableErr>(exp: bool, err: E) -> ProgramResult {
    if !exp {
        err.print();
        Err(err.err())
    } else {
        Ok(())
    }
}

pub trait PrintableErr {
    fn print(&self);
    fn err(self) -> ProgramError;
}

impl PrintableErr for ProgramError {
    fn print(&self) {
        PrintProgramError::print::<TokenError>(self);
    }

    fn err(self) -> ProgramError {
        self
    }
}

impl PrintableErr for ContractError {
    fn print(&self) {
        msg!("Error: {:?}", self);
    }

    fn err(self) -> ProgramError {
        ProgramError::from(self)
    }
}

impl PrintableErr for &str {
    fn print(&self) {
        msg!("Error: {}", self);
    }

    fn err(self) -> ProgramError {
        ProgramError::Custom(255)
    }
}
