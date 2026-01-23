use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self,Mint, Token, TokenAccount, Transfer},
};

use crate::{Escrow, Loan};

#[derive(Accounts)]
pub struct Withdrawl<'info>{
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
    pub fn withdrawl(
        &mut self,

    ) ->Result<()>{

        Ok(())
    }
}