use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{CollateralType, CreditError, Escrow, Loan};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"escrow", user.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"loan", user.key().as_ref()],
        bump,
    )]
    pub loan: Account<'info, Loan>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, collateral_type: CollateralType) -> Result<()> {
        require!(amount > 0, CreditError::InvalidAmount);
        require!(
            collateral_type != CollateralType::YetToSet,
            CreditError::InvalidCollateralType
        );
        require!(
            self.loan.collateral_mint == self.mint.key(),
            CreditError::InvalidCollateralMint
        );
        require!(
            self.loan.collateral_type == CollateralType::YetToSet
                || self.loan.collateral_type == collateral_type,
            CreditError::InvalidCollateralType
        );

        self.loan.collateral_amount = self
            .loan
            .collateral_amount
            .checked_add(amount)
            .ok_or(CreditError::AmountOverflow)?;
        self.loan.collateral_type = collateral_type;

        Ok(())
    }
}
