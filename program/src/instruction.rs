use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    GenerateVault,
}

impl PlatformInstruction {
    pub fn generate_vault(wallet_pubkey: Pubkey, program_id: Pubkey) -> Instruction {
        let (vault_pda, _) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::GenerateVault,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new(system_program::id(), false),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
            ],
        )
    }
}
