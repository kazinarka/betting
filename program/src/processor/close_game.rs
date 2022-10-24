use crate::consts::{ADMIN, BETTING, GAME, USER, WHITELIST};
use crate::error::ContractError;
use crate::processor::require;
use crate::state::helpers::{get_betting_info, get_game_info, get_user_info};
use crate::state::structs::{BettingInfo, Game, User};
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn close(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    user: Pubkey,
    winner_address: Pubkey,
    type_price: u64,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (betting_pda, _) = Pubkey::find_program_address(&[BETTING], program_id);

    if accounts.pda.key != &betting_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let betting_info = get_betting_info(&accounts.pda.data.borrow())?;

    if &betting_info.manager != accounts.payer.key {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (game_pda, game_bump) = Pubkey::find_program_address(&[GAME, &user.to_bytes()], program_id);

    if *accounts.game.key != game_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut game_info = get_game_info(&accounts.game.data.borrow())?;

    require(!game_info.closed, "Game already closed")?;
    require(
        winner_address == game_info.gamer1 || winner_address == game_info.gamer2,
        "invalid winner",
    )?;

    game_info.closed = true;

    let (user_pda, _) = Pubkey::find_program_address(&[USER, &user.to_bytes()], program_id);

    if *accounts.user.key != user_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (user2_pda, _) =
        Pubkey::find_program_address(&[USER, &game_info.gamer2.to_bytes()], program_id);

    if *accounts.user2.key != user2_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut user_info = get_user_info(&accounts.user.data.borrow())?;

    let mut user2_info = get_user_info(&accounts.user2.data.borrow())?;

    user_info.in_game = false;
    user_info.turnover += type_price;
    user_info.serialize(&mut &mut accounts.user.data.borrow_mut()[..])?;

    user2_info.in_game = false;
    user2_info.turnover += type_price;
    user2_info.serialize(&mut &mut accounts.user2.data.borrow_mut()[..])?;

    let total_fee = game_info.amount1 * betting_info.global_fee / 100;

    let looser_address = if winner_address == game_info.gamer1 {
        game_info.gamer2
    } else {
        game_info.gamer1
    };

    game_info.serialize(&mut &mut accounts.game.data.borrow_mut()[..])?;

    let (token_pda, _) =
        Pubkey::find_program_address(&[WHITELIST, &accounts.token.key.to_bytes()], program_id);

    if *accounts.supported_token.key != token_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (token1_pda, _) =
        Pubkey::find_program_address(&[WHITELIST, &accounts.token1.key.to_bytes()], program_id);

    if *accounts.supported_token1.key != token1_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&game_pda, accounts.token.key)
        != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&game_pda, accounts.token1.key)
        != accounts.source1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &game_info.gamer1,
        accounts.token.key,
    ) != accounts.user_destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &game_info.gamer1,
        accounts.token1.key,
    ) != accounts.user_destination1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &user_info.referrer,
        accounts.token.key,
    ) != accounts.referrer_dest.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &user2_info.referrer,
        accounts.token.key,
    ) != accounts.referrer1_dest.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &user_info.referrer,
        accounts.token1.key,
    ) != accounts.referrer_dest1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &user2_info.referrer,
        accounts.token1.key,
    ) != accounts.referrer1_dest1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if *accounts.referrer.key != user_info.referrer {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if *accounts.referrer1.key != user2_info.referrer {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &game_info.gamer2,
        accounts.token.key,
    ) != accounts.user1_destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        &game_info.gamer2,
        accounts.token1.key,
    ) != accounts.user1_destination1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if &spl_associated_token_account::get_associated_token_address(&admin, accounts.token.key)
        != accounts.owner_assoc.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&admin, accounts.token1.key)
        != accounts.owner_assoc1.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    internal_transfer(
        &accounts,
        winner_address,
        looser_address,
        game_info.token1,
        game_info.amount1,
        total_fee * 2,
        user_info.clone(),
        user2_info.clone(),
        game_bump,
        user,
        game_info.clone(),
        betting_info.clone(),
    )?;

    internal_transfer(
        &accounts,
        winner_address,
        looser_address,
        game_info.token2,
        game_info.amount2,
        0,
        user_info,
        user2_info.clone(),
        game_bump,
        user,
        game_info,
        betting_info,
    )?;

    Ok(())
}

fn internal_transfer(
    accounts: &Accounts,
    winner_address: Pubkey,
    looser_address: Pubkey,
    token_address: Pubkey,
    value: u64,
    fee: u64,
    user_info: User,
    user2_info: User,
    game_bump: u8,
    user: Pubkey,
    game_info: Game,
    betting_info: BettingInfo,
) -> ProgramResult {
    let accounts_token = if &token_address == accounts.token.key {
        accounts.token.clone()
    } else {
        accounts.token1.clone()
    };

    let accounts_source = if &token_address == accounts.token.key {
        accounts.source.clone()
    } else {
        accounts.source1.clone()
    };

    let accounts_owner_assoc = if &token_address == accounts.token.key {
        accounts.owner_assoc.clone()
    } else {
        accounts.owner_assoc1.clone()
    };

    let accounts_winner_assoc = if winner_address == game_info.gamer1 {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.user_destination.clone()
        } else {
            accounts.user_destination1.clone()
        }
    } else {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.user1_destination.clone()
        } else {
            accounts.user1_destination1.clone()
        }
    };

    let accounts_winner_referrer_assoc = if winner_address == game_info.gamer1 {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.referrer_dest.clone()
        } else {
            accounts.referrer_dest1.clone()
        }
    } else {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.referrer1_dest.clone()
        } else {
            accounts.referrer1_dest1.clone()
        }
    };

    let accounts_looser_referrer_assoc = if looser_address == game_info.gamer1 {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.referrer_dest.clone()
        } else {
            accounts.referrer_dest1.clone()
        }
    } else {
        if accounts_token.key.clone() == accounts.token.key.clone() {
            accounts.referrer1_dest.clone()
        } else {
            accounts.referrer1_dest1.clone()
        }
    };

    let (accounts_winner_user, winner_user_info) = if winner_address == game_info.gamer1 {
        (accounts.user_wallet.clone(), user_info.clone())
    } else {
        (accounts.user1_wallet.clone(), user2_info.clone())
    };

    let looser_user_info = if looser_address == game_info.gamer1 {
        user_info
    } else {
        user2_info
    };

    let accounts_winner_referrer = if &winner_user_info.referrer == accounts.referrer.key {
        accounts.referrer.clone()
    } else {
        accounts.referrer1.clone()
    };

    let accounts_looser_referrer = if &looser_user_info.referrer == accounts.referrer.key {
        accounts.referrer.clone()
    } else {
        accounts.referrer1.clone()
    };

    if fee != 0 {
        if winner_user_info.referrer == Pubkey::default()
            && looser_user_info.referrer == Pubkey::default()
        {
            if accounts_owner_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts.owner.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_owner_assoc.clone(),
                        accounts.owner.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_owner_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.admin_fee + betting_info.referrer_fee) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_owner_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;
        } else if winner_user_info.referrer == Pubkey::default() {
            if accounts_owner_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts.owner.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_owner_assoc.clone(),
                        accounts.owner.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_owner_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.admin_fee + betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_owner_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;

            if accounts_looser_referrer_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts_looser_referrer.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_looser_referrer_assoc.clone(),
                        accounts_looser_referrer.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_looser_referrer_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_looser_referrer_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;
        } else if looser_user_info.referrer == Pubkey::default() {
            if accounts_owner_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts.owner.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_owner_assoc.clone(),
                        accounts.owner.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_owner_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.admin_fee + betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_owner_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;

            if accounts_winner_referrer_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts_winner_referrer.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_winner_referrer_assoc.clone(),
                        accounts_winner_referrer.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_winner_referrer_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_winner_referrer_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;
        } else {
            if accounts_owner_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts.owner.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_owner_assoc.clone(),
                        accounts.owner.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_owner_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * betting_info.admin_fee / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_owner_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;

            if accounts_winner_referrer_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts_winner_referrer.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_winner_referrer_assoc.clone(),
                        accounts_winner_referrer.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_winner_referrer_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_winner_referrer_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;

            if accounts_looser_referrer_assoc.owner != accounts.token_program.key {
                invoke(
                    &spl_associated_token_account::create_associated_token_account(
                        accounts.payer.key,
                        &accounts_looser_referrer.key,
                        accounts_token.key,
                    ),
                    &[
                        accounts.payer.clone(),
                        accounts_looser_referrer_assoc.clone(),
                        accounts_looser_referrer.clone(),
                        accounts_token.clone(),
                        accounts.system_program.clone(),
                        accounts.token_program.clone(),
                        accounts.rent_info.clone(),
                        accounts.token_assoc.clone(),
                    ],
                )?;
            }

            invoke_signed(
                &spl_token::instruction::transfer(
                    accounts.token_program.key,
                    accounts_source.key,
                    accounts_looser_referrer_assoc.key,
                    accounts.game.key,
                    &[],
                    fee * (betting_info.referrer_fee / 2) / 100,
                )?,
                &[
                    accounts_source.clone(),
                    accounts_looser_referrer_assoc.clone(),
                    accounts.game.clone(),
                    accounts.token_program.clone(),
                ],
                &[&[GAME, &user.to_bytes(), &[game_bump]]],
            )?;
        }
    }

    if accounts_winner_assoc.owner != accounts.token_program.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                accounts.payer.key,
                &accounts_winner_user.key,
                accounts_token.key,
            ),
            &[
                accounts.payer.clone(),
                accounts_winner_assoc.clone(),
                accounts_winner_user.clone(),
                accounts_token.clone(),
                accounts.system_program.clone(),
                accounts.token_program.clone(),
                accounts.rent_info.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            accounts.token_program.key,
            accounts_source.key,
            accounts_winner_assoc.key,
            accounts.game.key,
            &[],
            value - fee,
        )?,
        &[
            accounts_source.clone(),
            accounts_winner_assoc.clone(),
            accounts.game.clone(),
            accounts.token_program.clone(),
        ],
        &[&[GAME, &user.to_bytes(), &[game_bump]]],
    )?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub supported_token: &'a AccountInfo<'b>,
    pub supported_token1: &'a AccountInfo<'b>,
    pub user: &'a AccountInfo<'b>,
    pub user2: &'a AccountInfo<'b>,
    pub user_wallet: &'a AccountInfo<'b>,
    pub user1_wallet: &'a AccountInfo<'b>,
    pub game: &'a AccountInfo<'b>,
    pub source: &'a AccountInfo<'b>,
    pub source1: &'a AccountInfo<'b>,
    pub user_destination: &'a AccountInfo<'b>,
    pub user_destination1: &'a AccountInfo<'b>,
    pub user1_destination: &'a AccountInfo<'b>,
    pub user1_destination1: &'a AccountInfo<'b>,
    pub referrer: &'a AccountInfo<'b>,
    pub referrer1: &'a AccountInfo<'b>,
    pub referrer_dest: &'a AccountInfo<'b>,
    pub referrer1_dest: &'a AccountInfo<'b>,
    pub referrer_dest1: &'a AccountInfo<'b>,
    pub referrer1_dest1: &'a AccountInfo<'b>,
    pub owner: &'a AccountInfo<'b>,
    pub owner_assoc: &'a AccountInfo<'b>,
    pub owner_assoc1: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub token: &'a AccountInfo<'b>,
    pub token1: &'a AccountInfo<'b>,
    pub token_assoc: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            pda: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            supported_token: next_account_info(acc_iter)?,
            supported_token1: next_account_info(acc_iter)?,
            user: next_account_info(acc_iter)?,
            user2: next_account_info(acc_iter)?,
            user_wallet: next_account_info(acc_iter)?,
            user1_wallet: next_account_info(acc_iter)?,
            game: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            source1: next_account_info(acc_iter)?,
            user_destination: next_account_info(acc_iter)?,
            user_destination1: next_account_info(acc_iter)?,
            user1_destination: next_account_info(acc_iter)?,
            user1_destination1: next_account_info(acc_iter)?,
            referrer: next_account_info(acc_iter)?,
            referrer1: next_account_info(acc_iter)?,
            referrer_dest: next_account_info(acc_iter)?,
            referrer1_dest: next_account_info(acc_iter)?,
            referrer_dest1: next_account_info(acc_iter)?,
            referrer1_dest1: next_account_info(acc_iter)?,
            owner: next_account_info(acc_iter)?,
            owner_assoc: next_account_info(acc_iter)?,
            owner_assoc1: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            token: next_account_info(acc_iter)?,
            token1: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
        })
    }
}
