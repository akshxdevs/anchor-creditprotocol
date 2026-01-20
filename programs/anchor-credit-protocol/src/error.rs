use anchor_lang::prelude::*;

#[error_code]
pub enum CreditError {
    #[msg("Invalid amount: must be greater than zero")]
    InvalidAmount,
    #[msg("Invalid collateral type")]
    InvalidCollateralType,
    #[msg("Invalid timestamp: start time must be before due time")]
    InvalidTimestamp,
    #[msg("Invalid collateral mint: does not match loan collateral mint")]
    InvalidCollateralMint,
    #[msg("Overflow error: amount exceeds maximum value")]
    AmountOverflow,
}
