use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Exchange Ratio")]
    InvalidExchangeRatio,

    #[msg("Invalid Admin")]
    InvalidAdmin,

    #[msg("Invalid New Admin")]
    InvalidNewAdmin,

    #[msg("Invalid New Exchange Ratio Operator")]
    InvalidNewExchangeRatioOperator,

    #[msg("Invalid Exchange Ratio Operator")]
    InvalidExchangeRatioOperator,

    #[msg("Mint and Burn Already Paused")]
    MintAndBurnAlreadyPaused,

    #[msg("Mint and Burn Not Paused")]
    MintAndBurnNotPaused,

    #[msg("Invalid Amount")]
    InvalidAmount,

    #[msg("Max Supply Reached")]
    MaxSupplyReached,

    #[msg("Less Than Previous Max Supply")]
    LessThanPreviousMaxSupply,
}
