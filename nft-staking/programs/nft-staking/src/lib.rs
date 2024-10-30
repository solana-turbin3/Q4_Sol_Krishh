use anchor_lang::prelude::*;

declare_id!("CnHh8FuPfKm1pqwssQGBE7Av1z2NdQjaaQDEUo5AJKEK");

pub mod states;
pub mod instructions;
pub mod errors;

use instructions::{
    init_stake::*,
    init_stake_configs::*,
    init_user_account::*,
    unstake::*,
    claim::*
};

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitStakeConfigs>, points_per_stake: u8, max_stake: u8, freeze_period: u32 ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, points_per_stake, max_stake, freeze_period)
    }

    pub fn init_acocunt(ctx: Context<InitUserAccount>) -> Result<()> {
        ctx.accounts.init_account(&ctx.bumps)
    }

    pub fn init_stake_account(ctx: Context<InitStakeAccount>)-> Result<()> {
        ctx.accounts.init_stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>)-> Result<()> {
        ctx.accounts.claim()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
