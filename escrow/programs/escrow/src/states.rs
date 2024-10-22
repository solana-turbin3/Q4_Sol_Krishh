use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub vault_bump: u8,
    pub state_bump: u8
}