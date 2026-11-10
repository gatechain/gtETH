use std::mem::size_of;

use crate::constants::{SEED_AUTH_STORE, SEED_MINT, SEED_GTBTC_AUTHORITY};
use crate::states::{AuthStore, InitializeEvent};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instructions::CreateMetadataAccountV3CpiBuilder;
use mpl_token_metadata::types::DataV2;

#[derive(Accounts)]
#[instruction(
    params: InitializeParams
)]
pub struct Initialize<'info> {
    #[account(mut,)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer = admin, 
        space = 8 + size_of::<AuthStore>(),
        seeds=[SEED_AUTH_STORE],
        bump,
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 9, // NOTE: gtDOGE=8, gtXRP=6, gtBTC=8, gtETH=9,
        mint::authority = gtbtc_authority.key(),
        mint::freeze_authority = admin.key(),
        seeds = [SEED_MINT, params.gtbtc_mint_seed.as_bytes().as_ref()],
        bump,
    )]
    pub gtbtc_mint: Account<'info, Mint>,

    /// CHECK: This is the metadata account for the mint
    #[account(
        mut,
        seeds = [
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            gtbtc_mint.key().as_ref(),
        ],
        bump,
        seeds::program = mpl_token_metadata::ID,
    )]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_GTBTC_AUTHORITY],
        bump,
    )]
    pub gtbtc_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is the token metadata program
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeParams {
    pub operator_pub_key: Pubkey,
    pub gtbtc_mint_seed: String,
    pub max_supply: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let auth_store = &mut ctx.accounts.auth_store.load_init()?;
    // Only allowed to be initialized once
    require_keys_eq!(auth_store.admin_key.key(), Pubkey::default());
    require_keys_neq!(params.operator_pub_key, Pubkey::default());

    auth_store.admin_key = ctx.accounts.admin.key();   
    auth_store.exchange_ratio_operator = params.operator_pub_key;
    auth_store.is_paused = 0; // 0: not paused, 1: paused
    auth_store.exchange_ratio = 1000000; 
    auth_store.exchange_ratio_decimals = 6;
    auth_store.max_supply = params.max_supply;

    // Create metadata for the token
    let seeds = &[
        SEED_GTBTC_AUTHORITY,
        &[ctx.bumps.gtbtc_authority],
    ];
    let signer = [&seeds[..]];

    let data_v2 = DataV2 {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    CreateMetadataAccountV3CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .metadata(&ctx.accounts.metadata)
        .mint(&ctx.accounts.gtbtc_mint.to_account_info())
        .mint_authority(&ctx.accounts.gtbtc_authority)
        .payer(&ctx.accounts.admin)
        .update_authority(&ctx.accounts.gtbtc_authority, true)
        .system_program(&ctx.accounts.system_program)
        .data(data_v2)
        .is_mutable(true)
        .invoke_signed(&signer)?;

    emit!(InitializeEvent {
        admin_pub_key: auth_store.admin_key,
        exchange_ratio_operator_pub_key: auth_store.exchange_ratio_operator,
        is_paused: auth_store.is_paused,
        exchange_ratio: auth_store.exchange_ratio,
        max_supply: auth_store.max_supply,
    });

    Ok(())
}



