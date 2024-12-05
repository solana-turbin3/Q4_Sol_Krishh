use anchor_lang::prelude::*;
use crate::states::config::Config;
use anchor_spl::token_interface::{ TokenInterface, Mint };

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 1,
        seeds = [b"config".as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        seeds = [b"payment_token", admin.key.as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> InitConfig<'info> {

    pub fn config_init(&mut self, bumps: &InitConfigBumps) -> Result<()> {
        
        // Initialize a config account
        self.config.set_inner( Config {
            admin: self.admin.key(),
            bump: bumps.config,
            token_mint_bump: bumps.token_mint
        });

        Ok(())
    }
}