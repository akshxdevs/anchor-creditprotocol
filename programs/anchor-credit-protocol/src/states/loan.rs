use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Loan {
    pub borrower: Pubkey,
    pub lender: Pubkey,
    pub principal: u64,
    pub interest_bps: u16,
    pub collateral_mint: Pubkey,
    pub collateral_amount: u64,
    pub start_ts: i64,
    pub due_ts: i64,
    pub status: LoanStatus,
    pub collateral_type: CollateralType,
    pub existing_user: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum LoanStatus {
    Requested,
    Active,
    Repaid,
    Defaulted,
    Liquidated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum CollateralType {
    SOL,
    USDC,
    AkshToken,
    YetToSet,
}
