use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{ MasterEditionAccount, Metadata, MetadataAccount}, token_interface::{
    Mint, 
    TokenAccount,
    TokenInterface
}};

use crate::{
    errors::ERRORS, states::{listing::Listing, marketplace::MarketPlace}
};

#[derive(Accounts)]
pub struct InitializeListings<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    pub marketplace: Account<'info, MarketPlace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::authority = listing,
        associated_token::mint = maker_mint
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init, 
        payer = maker, 
        seeds = [marketplace.key().as_ref(), maker.key().as_ref() ],
        bump,
        space = Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>
}   