use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::{CreditError, Escrow, Loan, LoanList, UserProfile};

#[derive(Accounts)]
pub struct Withdrawl<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info,UserProfile>,
    #[account(
        mut,
        seeds = [b"loan_list", user.key().as_ref()],
        bump,
    )]
    pub loan_list: Account<'info,LoanList>,
    #[account(
        mut,
        seeds = [b"escrow", user.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

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

impl <'info> Withdrawl <'info>{
    pub fn issue_loan_amount(
        &mut self,
    ) ->Result<()>{
        require!(self.user_profile.total_defaults == 0, CreditError::UserHasDefaults);
        require!(self.loan_list.loan_list.len() != 0 , CreditError::NoActiveLoans);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let user_key = self.user.key();
        let seeds = &[
            b"escrow",
            user_key.as_ref(),
            &[self.escrow.bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);
        token::transfer(ctx, self.loan.collateral_amount)?;        
        Ok(())
    }
}