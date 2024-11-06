use anchor_lang::prelude::*;

use crate::states::marketplace::MarketPlace;
use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        space = 8 +  MarketPlace::INIT_SPACE,
        seeds = [b"maekteplace", name.as_bytes()],
        bump, 
        payer = admin
    )]
    pub marketplace: Account<'info, MarketPlace>,
    #[account(
        seeds = [b"treasury",  marketplace.key().as_ref()],
        bump
    )]
    pub trasury: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn init_marketplace(
        &mut self,
        name: String,
        bumps: &InitializeMarketplaceBumps,
        fee: u16,
    ) -> Result<()> {
        self.marketplace.set_inner(MarketPlace {
            admin: self.admin.key(),
            bump: bumps.marketplace,
            fee,
            reward_bump: bumps.reward_mint,
            treasury_bump: bumps.trasury,
            name,
        });

        todo!()
    }
}
