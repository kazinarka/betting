pub mod generate_vault;

use crate::error::ContractError;
use crate::instruction::PlatformInstruction;
use crate::processor::generate_vault::generate_vault;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: PlatformInstruction =
            match PlatformInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            PlatformInstruction::GenerateVault => generate_vault(accounts, program_id)?,
        };

        Ok(())
    }
}
