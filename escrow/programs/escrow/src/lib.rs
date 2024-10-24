use anchor_lang::prelude::*;

declare_id!("HANtaDDvQZupaFkWL1NPumiV5ZgRpVzBLwLAPkEUuvpm");
pub mod states;
pub mod instructions;
pub mod error;
use instructions::{
    initialize::*,
    refund::*,
    take::*
};

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {

        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund> ) -> Result<()> {
        ctx.accounts.refund_and_clean_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit_b_amount()?;
        ctx.accounts.withdraw_a_amount_and_close_vault()
    }
}

