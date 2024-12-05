use anchor_lang::prelude::*;

#[account]
pub struct UserState {
    pub points: u32, // the points user redeem in their vulnerable gameplay.
    pub user_bump: u8
}