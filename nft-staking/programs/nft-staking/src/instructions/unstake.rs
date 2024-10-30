use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{ mpl_token_metadata::instructions::{ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, 
    token::{ revoke, Mint, Revoke, Token, TokenAccount}
};

use crate::{states::{StakeAccount::StakeAccount, StakeConfig::StakeConfig, UserAccount::UserAccount}, errors::ERROR};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = collection.key().as_ref() == metadata.collection.as_ref().unwrap().key.as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key(),
    )]  
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds= [b"config".as_ref()],
        bump = config.bump
    )]
    pub config : Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user_account".as_ref(), user.key().as_ref()],
        bump = user_account.bump
     )]
     pub user_account: Account<'info, UserAccount>,
     #[account(
        mut,
        close = user,
        seeds= [b"stake_account".as_ref(), config.key().as_ref(), user.key().as_ref()],
        bump,
     )]
     pub stake_account: Account<'info, StakeAccount>,
     pub system_program: Program<'info, System>,
     pub metadata_program: Program<'info, Metadata>,
     pub token_account: Program<'info, Token>,    
}

impl<'info> Unstake<'info>{
    pub fn unstake(&mut self) -> Result<()> {

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;

        require!(time_elapsed >= self.config.freeze_period, ERROR::StakingPeriodNotElapsed);

        self.user_account.point += time_elapsed as u32 * self.config.points_per_stake as u32;

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_account.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        
        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            }
        ).invoke_signed(signer_seeds)?;

        let cpi_program = self.token_account.to_account_info();

        let cpi_accounts =  Revoke{
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_ctx)?;

        self.user_account.amount_staked -= 1;

        Ok(())
    }
}