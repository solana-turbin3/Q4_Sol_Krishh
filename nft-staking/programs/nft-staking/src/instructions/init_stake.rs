use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{
        approve, Approve, Mint, Token, TokenAccount
    }
};

use crate::{
    states::{StakeAccount::StakeAccount, StakeConfig::StakeConfig, UserAccount::UserAccount},
    errors::ERROR
};

#[derive(Accounts)]
pub struct InitStakeAccount<'info> {
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
        seeds = [b"user_account".as_ref(), user.key().as_ref()],
        bump = user_account.bump
     )]
     pub user_account: Account<'info, UserAccount>,
     #[account(
        init, 
        space = StakeAccount::INIT_SPACE,
        seeds= [b"stake_account".as_ref(), config.key().as_ref(), user.key().as_ref()],
        bump,
        payer= user
     )]
     pub stake_account: Account<'info, StakeAccount>,
     pub system_program: Program<'info, System>,
     pub metadata_program: Program<'info, Metadata>,
     pub token_account: Program<'info, Token>,    
}



impl<'info> InitStakeAccount<'info> {
    pub fn init_stake(&mut self, bumps: &InitStakeAccountBumps)-> Result<()> {

        require!(self.user_account.amount_staked < self.config.max_stake, ERROR::MaxStakeLimitReached);

        self.stake_account.set_inner(StakeAccount{
            bump: bumps.stake_account,
            staked_at: Clock::get()?.unix_timestamp,
            mint: self.mint.key(),
            owner: self.user.key()
        });

        let approve_accounts = Approve {
            authority: self.user.to_account_info(),
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info()
        };

        let delegate_ctx = CpiContext::new(self.token_account.to_account_info(), approve_accounts);

        approve(delegate_ctx, 1)?;

        let seeds = &[
            b"stake_account",
            self.config.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref()
        ];
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_program = &self.token_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts{ 
                delegate,
                edition,
                token_account,  
                mint,
                token_program
            }
        ).invoke_signed(signer_seeds)?;

        self.user_account.amount_staked += 1;

        Ok(())
    }
}