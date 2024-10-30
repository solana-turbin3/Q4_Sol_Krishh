use anchor_lang::prelude::*;

declare_id!("CnHh8FuPfKm1pqwssQGBE7Av1z2NdQjaaQDEUo5AJKEK");

pub mod states;
pub mod instructions;
pub mod errors;

use instructions::{
    init_stake::*,
    init_stake_configs::*,
    init_user_account::*
};

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitStakeConfigs>, points_per_stake: u8, max_stake: u8, freeze_period: u32 ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, points_per_stake, max_stake, freeze_period)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
