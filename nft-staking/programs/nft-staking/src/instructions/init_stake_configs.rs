use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::states::StakeConfig::StakeConfig;

#[derive(Accounts)]
pub struct InitStakeConfigs<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [b"configs".as_ref()],
        payer = admin,
        bump,
        space = 8 + StakeConfig::INIT_SPACE
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"reward_mint".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config       
    )]
    pub reward_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> InitStakeConfigs<'info> {
    pub fn init(&mut self, bumps: &InitStakeConfigsBumps,points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        self.config.set_inner(StakeConfig{
            bump: bumps.config,
            max_stake,
            freeze_period,
            points_per_stake,
            reward_bump: bumps.reward_mint
        });

        Ok(())
    }
}