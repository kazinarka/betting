use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    Init {
        manager: Pubkey,
        supported_token: Pubkey,
        is_stablecoin: bool,
    },
    ChangeCloseDelay {
        new_delay: u64,
    },
}
