use anchor_lang::prelude::*;
use crate::{
    errors::*,
    states::config::Configs 
};

use anchor_spl::{
    token_interface::{
        Mint, 
        TokenAccount,
        TokenInterface
    },
    associated_token::AssociatedToken
};

use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = auth,
      )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint_b,
        associated_token::authority = auth,
     )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECK This is okay
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,

    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority = lp
    )]
    pub lp_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut, 
        associated_token::mint = mint_b,
        associated_token::authority = lp
    )]  
    pub lp_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = lp,
        seeds = [b"mint_lp",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [b"config", config.seeds.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Configs>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    
}

