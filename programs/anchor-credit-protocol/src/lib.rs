use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod states;

pub use error::*;
// pub use event::*;
pub use instructions::*;
pub use states::*;

declare_id!("3hCQwkaJM5ePHyT2YDnCxE5K8DT1WpucB16yuJnMcpde");

#[program]
pub mod anchor_credit_protocol {
    use super::*;
    pub fn deposite(
        ctx: Context<Deposit>,
        amount:u64,
        collateral_type:CollateralType,
    ) -> Result<()>{
        ctx.accounts.deposit(
            amount, 
            collateral_type
        )?;
        Ok(())
    }
    pub fn initialize_loan(
        ctx: Context<Initialize>,
        principal: u64,
        interest_bps: u16,
        collateral_mint: Pubkey,
        collateral_amount: u64,
        due_ts: u32,
        existing_user: bool,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize_loan(
            principal,
            interest_bps,
            collateral_mint,
            collateral_amount,
            due_ts,
            existing_user,
        )?;
        Ok(())
    }


}
