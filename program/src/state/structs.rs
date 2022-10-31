use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Game {
    pub gamer1: Pubkey,
    pub gamer2: Pubkey,
    pub token1: Pubkey,
    pub token2: Pubkey,
    pub amount1: u64,
    pub amount2: u64,
    pub latest_bet: u64,
    pub closed: bool,
    pub type_price: u64,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BettingInfo {
    pub referrer_fee: u64,
    pub admin_fee: u64,
    pub global_fee: u64,
    pub transaction_fee: u64,
    pub accept_bets: bool,
    pub close_delay: u64,
    pub manager: Pubkey,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct SupportedToken {
    pub mint: Pubkey,
    pub feed: Pubkey,
    pub is_stablecoin: bool,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct User {
    pub address: Pubkey,
    pub referrer: Pubkey,
    pub in_game: bool,
    pub support_bots: bool,
    pub is_bot: bool,
    pub turnover: u64,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct TypePrice {
    pub price: u64,
}
