use anchor_lang::prelude::*;

declare_id!("9zmQYSuFqX5DeFkR7K6D4W2HaKWvkPmaGrVrgMTQqFDw");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod helper;
pub mod errors;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
