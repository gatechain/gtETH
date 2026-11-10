use anchor_lang::prelude::*;

#[event]
pub struct InitializeEvent {
    pub admin_pub_key: Pubkey,
    pub exchange_ratio_operator_pub_key: Pubkey,
    pub is_paused: u8,
    pub exchange_ratio: u64,
    pub max_supply: u64,
}

#[event]
pub struct MintGtbtcEvent {
    pub mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct BurnGtbtcEvent {
    pub mint: Pubkey,
    pub source_token_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct MintAndBurnStatusEvent {
    pub is_paused: u8,
}

#[event]
pub struct UpdateExchangeRatioEvent {
    pub previous_exchange_ratio: u64,
    pub exchange_ratio: u64,
}

#[event]
pub struct UpdateMaxSupplyEvent {
    pub previous_max_supply: u64,
    pub max_supply: u64,
}

#[event]
pub struct UpdateAdminEvent {
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct UpdateExchangeRatioOperatorEvent {
    pub old_operator: Pubkey,
    pub new_operator: Pubkey,
}