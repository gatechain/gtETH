use crate::constants::{ SEED_AUTH_STORE};
use anchor_lang::prelude::*;
use crate::states::{AuthStore, MintAndBurnStatusEvent, UpdateAdminEvent, UpdateExchangeRatioEvent, UpdateExchangeRatioOperatorEvent, UpdateMaxSupplyEvent};
use crate::error::ErrorCode;

// ------------------------------------------------------------
// Update Mint and Burn Status
// ------------------------------------------------------------

#[derive(Accounts)]
pub struct UpdateByAdminAccounts<'info> {
    #[account(
        mut,
        constraint = admin.key() == auth_store.load()?.admin_key @ErrorCode::InvalidAdmin,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut, 
        seeds=[SEED_AUTH_STORE],
        bump,
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,
}

pub fn pause_mint_and_burn(
    ctx: Context<UpdateByAdminAccounts>,
) -> Result<()> {
    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    require!(auth_store.is_paused == 0, ErrorCode::MintAndBurnAlreadyPaused);
    auth_store.is_paused = 1;

    emit!(MintAndBurnStatusEvent {
        is_paused: auth_store.is_paused,
    });
    Ok(())
}

pub fn resume_mint_and_burn(
    ctx: Context<UpdateByAdminAccounts>,
) -> Result<()> {
    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    require!(auth_store.is_paused == 1, ErrorCode::MintAndBurnNotPaused);
    auth_store.is_paused = 0;

    emit!(MintAndBurnStatusEvent {
        is_paused: auth_store.is_paused,
    });
    Ok(())
}

pub fn update_admin(
    ctx: Context<UpdateByAdminAccounts>,
    new_admin: Pubkey,
) -> Result<()> {
    require!(new_admin.key() != ctx.accounts.admin.key() && new_admin.key() != Pubkey::default(), ErrorCode::InvalidNewAdmin);
    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    let old_admin = auth_store.admin_key;
    auth_store.admin_key = new_admin;

    emit!(UpdateAdminEvent {
        old_admin: old_admin,
        new_admin: new_admin,
    });
    Ok(())
}

pub fn update_exchange_ratio_operator(
    ctx: Context<UpdateByAdminAccounts>,
    new_operator: Pubkey,
) -> Result<()> {
    require!(new_operator.key() != Pubkey::default(), ErrorCode::InvalidNewExchangeRatioOperator);

    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    require!(new_operator.key() != auth_store.exchange_ratio_operator.key(), ErrorCode::InvalidNewExchangeRatioOperator);
    
    let old_operator = auth_store.exchange_ratio_operator;
    auth_store.exchange_ratio_operator = new_operator;

    emit!(UpdateExchangeRatioOperatorEvent {
        old_operator: old_operator,
        new_operator: new_operator,
    });
    Ok(())
}

// ------------------------------------------------------------
// Update Max Supply
// ------------------------------------------------------------
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdateMaxSupplyParams {
    pub max_supply: u64,
}


pub fn update_max_supply(
    ctx: Context<UpdateByAdminAccounts>,
    params: UpdateMaxSupplyParams,
) -> Result<()> {
    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    let previous_max_supply = auth_store.max_supply;
    require!(params.max_supply > previous_max_supply, ErrorCode::LessThanPreviousMaxSupply);
    auth_store.max_supply = params.max_supply;

    emit!(UpdateMaxSupplyEvent {
        previous_max_supply: previous_max_supply,
        max_supply: auth_store.max_supply,
    });
    Ok(())
}


// ------------------------------------------------------------
// Update Exchange Ratio
// ------------------------------------------------------------

#[derive(Accounts)]
pub struct UpdateExchangeRatioAccounts<'info> {
    #[account(
        mut,
        constraint = operator.key() == auth_store.load()?.exchange_ratio_operator @ErrorCode::InvalidExchangeRatioOperator,
    )]
    pub operator: Signer<'info>,

    #[account(
        mut, 
        seeds=[SEED_AUTH_STORE],
        bump,
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdateExchangeRatioParams {
    pub exchange_ratio: u64,
}

pub fn update_exchange_ratio(
    ctx: Context<UpdateExchangeRatioAccounts>,
    params: UpdateExchangeRatioParams,
) -> Result<()> {
    require!(params.exchange_ratio >=1000000, ErrorCode::InvalidExchangeRatio); // 1000000 = 10^decimals = 10^6

    let mut auth_store = ctx.accounts.auth_store.load_mut()?;
    let previous_exchange_ratio = auth_store.exchange_ratio;
    auth_store.exchange_ratio = params.exchange_ratio;

    emit!(UpdateExchangeRatioEvent {
        previous_exchange_ratio: previous_exchange_ratio,
        exchange_ratio: auth_store.exchange_ratio,
    });
    Ok(())
}

