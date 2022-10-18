use crate::error::ContractError;
use crate::state::structs::{BettingInfo, Game, SupportedToken, User};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use std::cell::Ref;

pub fn get_betting_info(data: &Ref<&mut [u8]>) -> Result<BettingInfo, ProgramError> {
    if let Ok(data) = BettingInfo::try_from_slice(data) {
        Ok(data)
    } else {
        Err(ContractError::DeserializeError.into())
    }
}

pub fn get_game_info(data: &Ref<&mut [u8]>) -> Result<Game, ProgramError> {
    if let Ok(data) = Game::try_from_slice(data) {
        Ok(data)
    } else {
        Err(ContractError::DeserializeError.into())
    }
}

pub fn get_user_info(data: &Ref<&mut [u8]>) -> Result<User, ProgramError> {
    if let Ok(data) = User::try_from_slice(data) {
        Ok(data)
    } else {
        Err(ContractError::DeserializeError.into())
    }
}

pub fn get_supported_token_info(data: &Ref<&mut [u8]>) -> Result<SupportedToken, ProgramError> {
    if let Ok(data) = SupportedToken::try_from_slice(data) {
        Ok(data)
    } else {
        Err(ContractError::DeserializeError.into())
    }
}
