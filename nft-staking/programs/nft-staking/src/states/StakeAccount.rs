use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct StakeAccount{ 
    pub owner: Pubkey,
    pub staked_at: i64,
    pub mint: Pubkey,
    pub bump: u8 
}