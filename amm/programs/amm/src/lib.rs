use anchor_lang::prelude::*;

declare_id!("9zmQYSuFqX5DeFkR7K6D4W2HaKWvkPmaGrVrgMTQqFDw");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod helper;
pub mod errors;

use instructions::{
    initialize::*,
    deposit::*,
    swap::*
};

#[program]
pub mod amm {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, seeds: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize(seeds, &ctx.bumps, fee)
    }

    pub fn deposit_tokens(ctx: Context<Deposit>, amount: u64, max_a: u64, max_b: u64, expiration: i64) -> Result<()> {
        ctx.accounts.deposit(amount, max_a, max_b, expiration)
    } 

    pub fn swap(ctx: Context<Swap>, is_a: bool, min_amt: u64, expiration: i64, amount: u64) -> Result<()> {
        ctx.accounts.swap(is_a, amount, min_amt, expiration)
    }
}


#[derive(Accounts)]
pub struct Initialize {}
