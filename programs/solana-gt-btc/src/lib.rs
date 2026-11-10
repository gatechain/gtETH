use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod constants;
pub mod error;
use instructions::*;                   

declare_id!("BC4JBj4RZunhqN6Ra7r7NhjW7ZakCbUJ4NWCuLzHgY7h");

#[program]
pub mod solana_gt_btc {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(ctx, params)
    }

    pub fn mint_gtbtc(ctx: Context<MintGtbtcAccounts>, params: MintGtbtcParams) -> Result<()> {
        instructions::mint_gtbtc(ctx, params)
    }

    pub fn burn_gtbtc(ctx: Context<BurnGtbtcAccounts>, params: BurnGtbtcParams) -> Result<()> {
        instructions::burn_gtbtc(ctx, params)
    }

    pub fn pause_mint_and_burn(ctx: Context<UpdateByAdminAccounts>) -> Result<()> {
        instructions::pause_mint_and_burn(ctx)
    }

    pub fn resume_mint_and_burn(ctx: Context<UpdateByAdminAccounts>) -> Result<()> {
        instructions::resume_mint_and_burn(ctx)
    }

    pub fn update_max_supply(ctx: Context<UpdateByAdminAccounts>, params: UpdateMaxSupplyParams) -> Result<()> {
        instructions::update_max_supply(ctx, params)
    }

    pub fn update_exchange_ratio(ctx: Context<UpdateExchangeRatioAccounts>, params: UpdateExchangeRatioParams) -> Result<()> {
        instructions::update_exchange_ratio(ctx, params)
    }

    pub fn update_admin(ctx: Context<UpdateByAdminAccounts>, new_admin: Pubkey) -> Result<()> {
        instructions::update_admin(ctx, new_admin)
    }

    pub fn update_exchange_ratio_operator(ctx: Context<UpdateByAdminAccounts>, new_operator: Pubkey) -> Result<()> {
        instructions::update_exchange_ratio_operator(ctx, new_operator)
    }
}

