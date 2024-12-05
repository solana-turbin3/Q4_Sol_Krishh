use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,
    pub name: String,
    pub points: u16,
}