use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::states::{StakeConfig::StakeConfig, UserAccount::UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds=  [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"configs".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        seeds = [b"reward_mint".as_ref(), config.key().as_ref()],
        bump = config.reward_bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = config
    )]
    pub reward_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.reward_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_context, self.user_account.point as u64 * 10_u64.pow(self.reward_mint.decimals as u32))?;

        self.user_account.point = 0;
        Ok(())
    }
}