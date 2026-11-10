use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Default, Debug)]
#[repr(C)]
pub struct AuthStore {
    pub admin_key: Pubkey,
    pub exchange_ratio_operator: Pubkey,
    pub exchange_ratio: u64, // GTBTC:BTC
 
    pub max_supply: u64,
    pub is_paused: u8, // 0: not paused, 1: paused
    pub exchange_ratio_decimals: u8,
}
