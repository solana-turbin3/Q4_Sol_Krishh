use anchor_lang::prelude::*;

declare_id!("HANtaDDvQZupaFkWL1NPumiV5ZgRpVzBLwLAPkEUuvpm");
pub mod states;
pub mod instructions;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
