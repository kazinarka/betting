use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum BettingInstruction {
    Init {
        #[allow(dead_code)]
        manager: Pubkey,
        #[allow(dead_code)]
        supported_token: Pubkey,
        #[allow(dead_code)]
        is_stablecoin: bool,
    },
    ChangeCloseDelay {
        #[allow(dead_code)]
        new_delay: u64,
    },
    LockBets,
    UnlockBets,
    AddSupportedToken {
        #[allow(dead_code)]
        supported_token: Pubkey,
        #[allow(dead_code)]
        is_stablecoin: bool,
    },
    Registration {
        #[allow(dead_code)]
        referrer: Pubkey,
        #[allow(dead_code)]
        password: String,
    },
    NewManager {
        #[allow(dead_code)]
        manager: Pubkey,
    },
    SetGlobalFee {
        #[allow(dead_code)]
        fee: u64,
    },
    SetAdminFee {
        #[allow(dead_code)]
        fee: u64,
    },
    SetWinnerFee {
        #[allow(dead_code)]
        fee: u64,
    },
    SetTransactionFee {
        #[allow(dead_code)]
        fee: u64,
    },
    AddBot {
        #[allow(dead_code)]
        bot: Pubkey,
    },
    NewGame {
        #[allow(dead_code)]
        value: u64,
        #[allow(dead_code)]
        support_bot: bool,
    },
    JoinGame {
        #[allow(dead_code)]
        value: u64,
        #[allow(dead_code)]
        support_bot: bool,
        #[allow(dead_code)]
        user_master: Pubkey,
    },
    ForcedClose {
        #[allow(dead_code)]
        user: Pubkey,
    },
    ManuallyClose,
    Close {
        #[allow(dead_code)]
        user: Pubkey,
        #[allow(dead_code)]
        winner_address: Pubkey,
        #[allow(dead_code)]
        type_price: u64,
    },
}
