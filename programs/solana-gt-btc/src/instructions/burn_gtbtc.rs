use crate::constants::{SEED_AUTH_STORE, SEED_MINT};
use crate::error::ErrorCode;
use crate::states::{AuthStore, BurnGtbtcEvent};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Burn};

#[derive(Accounts)]
#[instruction(
    params: BurnGtbtcParams
)]
pub struct BurnGtbtcAccounts<'info> {
    #[account(
        mut,
        constraint = admin.key() == auth_store.load()?.admin_key @ErrorCode::InvalidAdmin,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_MINT, params.gtbtc_mint_seed.as_bytes().as_ref()],
        bump,
    )]
    pub gtbtc_mint: Account<'info, Mint>,

    #[account(
        mut, 
        seeds=[SEED_AUTH_STORE],
        bump,
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,

    #[account(
        mut,
        token::mint = gtbtc_mint,
        token::authority = admin, // now, only allow to burn from adminTokenAccount
    )]
    pub source_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct BurnGtbtcParams {
    pub burn_amount: u64,
    pub gtbtc_mint_seed: String,
}

#[access_control(check_op_permission(&ctx))]
pub fn burn_gtbtc(ctx: Context<BurnGtbtcAccounts>, burn_gtbtc_params: BurnGtbtcParams) -> Result<()> {
    // Validate input parameters
    require!(burn_gtbtc_params.burn_amount > 0, ErrorCode::InvalidAmount);

    // burn GTBTC from source_token_account
    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.gtbtc_mint.to_account_info(),
                from: ctx.accounts.source_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        burn_gtbtc_params.burn_amount,
    )?;

    emit!(BurnGtbtcEvent {
        mint: ctx.accounts.gtbtc_mint.key(),
        source_token_account: ctx.accounts.source_token_account.key(),
        amount: burn_gtbtc_params.burn_amount,
    });
    Ok(())
}

fn check_op_permission<'info>(ctx: &Context<BurnGtbtcAccounts<'info>>) -> Result<()> {
    let auth_store = &ctx.accounts.auth_store.load()?;
    require!(auth_store.is_paused == 0, ErrorCode::MintAndBurnAlreadyPaused);
    Ok(())
}
