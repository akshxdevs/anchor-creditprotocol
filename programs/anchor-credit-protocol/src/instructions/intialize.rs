use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{CreditError, Escrow, Loan};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        seeds = [b"escrow", user.key().as_ref()],
        bump,
        space = Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds = [b"loan", user.key().as_ref()],
        bump,
        space = Loan::INIT_SPACE,
    )]
    pub loan: Account<'info, Loan>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_loan(
        &mut self,
        principal: u64,
        interest_bps: u16,
        collateral_mint: Pubkey,
        collateral_amount: u64,
        due_ts: u32,
        existing_user: bool,
    ) -> Result<()> {
        let clock = Clock::get()?.unix_timestamp;

        let due_in_seconds: i64 = match due_ts {
            60 | 300 | 1200 | 3600 | 7200 => due_ts as i64,
            _ => return err!(CreditError::InvalidTimestamp),
        };
        let due_ts: i64 = clock
            .checked_add(due_in_seconds)
            .ok_or(CreditError::InvalidTimestamp)?;

        require!(principal > 0, CreditError::InvalidAmount);
        require!(collateral_amount > 0, CreditError::InvalidAmount);
        require!(clock < due_ts, CreditError::InvalidTimestamp);

        self.loan.set_inner(Loan {
            borrower: self.user.key(),
            lender: Pubkey::default(),
            principal,
            interest_bps,
            collateral_mint,
            collateral_amount,
            start_ts: clock,
            due_ts,
            status: crate::LoanStatus::Requested,
            collateral_type: crate::CollateralType::YetToSet,
            existing_user,
        });

        Ok(())
    }
}
