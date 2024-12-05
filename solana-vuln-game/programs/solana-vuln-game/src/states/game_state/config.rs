use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub bump: u8,
    pub token_mint_bump: u8
}