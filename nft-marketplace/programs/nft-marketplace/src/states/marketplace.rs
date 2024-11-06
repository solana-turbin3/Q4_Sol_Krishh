use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub reward_bump: u8,
    pub fee: u16,
    pub admin: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    #[max_len(32)]
    pub name: String,
}
