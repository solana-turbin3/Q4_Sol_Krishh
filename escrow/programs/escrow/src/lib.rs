use anchor_lang::prelude::*;

declare_id!("HANtaDDvQZupaFkWL1NPumiV5ZgRpVzBLwLAPkEUuvpm");
pub mod states;
pub mod instructions;
use instructions::initialize::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {

        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed,&ctx.bumps)?;

        Ok(())
    }
}

