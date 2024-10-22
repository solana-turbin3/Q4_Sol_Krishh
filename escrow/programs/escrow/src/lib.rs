use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;

use states::EscrowState;
use instructions::{
    initialize_escrow::*,
    
};

declare_id!("FS22fXE6AnE72uJo8ZUqcbDLNFVkbjKTgYrbaiCzBhL9");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<InitializeEscrow>) -> Result<()> {
        ctx.accounts.new(ctx.bumps)
    }

    
}




