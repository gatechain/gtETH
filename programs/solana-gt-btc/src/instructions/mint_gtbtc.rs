use crate::constants::SEED_GTBTC_AUTHORITY;
use crate::error::ErrorCode;
use crate::states::MintGtbtcEvent;
use crate::{
    constants::{SEED_AUTH_STORE, SEED_MINT},
    states::AuthStore,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use anchor_spl::{
    token::{mint_to, MintTo, Token},
};

#[derive(Accounts)]
#[instruction(
    params: MintGtbtcParams
)]
pub struct MintGtbtcAccounts<'info> {
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
        seeds = [SEED_GTBTC_AUTHORITY],
        bump,
    )]
    pub gtbtc_authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[SEED_AUTH_STORE],
        bump
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,

    #[account(
        mut,
        token::mint = gtbtc_mint,
        token::authority = admin, // now, only allow to mint to adminTokenAccount
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MintGtbtcParams {
    pub amount: u64,
    pub gtbtc_mint_seed: String,
}

#[access_control(check_op_permission(&ctx))]
pub fn mint_gtbtc<'info>(
    ctx: Context<MintGtbtcAccounts<'info>>,
    mint_gtbtc_params: MintGtbtcParams,
) -> Result<()> {
    require!(mint_gtbtc_params.amount > 0, ErrorCode::InvalidAmount);

    let auth_store = &ctx.accounts.auth_store.load()?;
    let current_supply = ctx.accounts.gtbtc_mint.supply;
    require!(
        mint_gtbtc_params.amount <= auth_store.max_supply.saturating_sub(current_supply),
        ErrorCode::MaxSupplyReached
    );

    let seeds = &[
        SEED_GTBTC_AUTHORITY,
        &[ctx.bumps.gtbtc_authority],
    ];
    let signer = [&seeds[..]];

    // mint `amount` GTBTC to recipient_token_account
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.gtbtc_authority.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                mint: ctx.accounts.gtbtc_mint.to_account_info(),
            },
            &signer,
        ),
        mint_gtbtc_params.amount,
    )?;

    emit!(MintGtbtcEvent {
        mint: ctx.accounts.gtbtc_mint.key(),
        recipient_token_account: ctx.accounts.recipient_token_account.key(),
        amount: mint_gtbtc_params.amount,
    });

    Ok(())
}

fn check_op_permission<'info>(ctx: &Context<MintGtbtcAccounts<'info>>) -> Result<()> {
    let auth_store = &ctx.accounts.auth_store.load()?;
    require!(auth_store.is_paused == 0, ErrorCode::MintAndBurnAlreadyPaused);
    Ok(())
}
