use anchor_lang::prelude::*;

declare_id!("CnHh8FuPfKm1pqwssQGBE7Av1z2NdQjaaQDEUo5AJKEK");

pub mod states;
pub mod instructions;

use states::*;
use instruction::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
