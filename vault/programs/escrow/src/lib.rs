use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;

use instructions::{
    initialize_escrow::*,
    deposit::*,
    withdraw::*
};

declare_id!("FS22fXE6AnE72uJo8ZUqcbDLNFVkbjKTgYrbaiCzBhL9");

#[program]
pub mod escrow {

    use super::*;

    pub fn initialize(ctx: Context<InitializeEscrow>) -> Result<()> {
        ctx.accounts.new(ctx.bumps)
    }

    pub fn deposit(ctx: Context<DepositAccounts>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<WithdrawAccounts>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}




