use anchor_lang::{prelude::*, solana_program::stake::instruction};

pub mod errors;
pub mod instrucitons;
pub mod states;

use instrucitons::*;

declare_id!("25HguGejmsSEDUDYrKd7WQf21jtvYDFcSRJkXSPqcS3u");

#[program]
pub mod nft_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<InitializeMarketplace>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init_marketplace(name, &ctx.bumps, fee)     
    } 

    pub fn list(ctx: Context<InitializeListings>, price: u64) -> Result<()> {
        ctx.accounts.init_listing(&ctx.bumps, price)?;
        ctx.accounts.send_nft_to_vault()
    }
    
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_mint_vault()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_vault()
    }

    
    
}

#[derive(Accounts)]
pub struct Initialize {}
