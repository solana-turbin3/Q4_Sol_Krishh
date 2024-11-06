use anchor_lang::prelude::*;

pub mod states;
pub mod instrucitons;
pub mod errors;

declare_id!("25HguGejmsSEDUDYrKd7WQf21jtvYDFcSRJkXSPqcS3u");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
