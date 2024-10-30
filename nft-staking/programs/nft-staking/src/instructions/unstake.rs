use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{ MasterEditionAccount, Metadata, MetadataAccount}, 
    token::{ Mint, Token, TokenAccount
    }
};

use crate::states::{StakeAccount::StakeAccount, StakeConfig::StakeConfig, UserAccount::UserAccount};

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
