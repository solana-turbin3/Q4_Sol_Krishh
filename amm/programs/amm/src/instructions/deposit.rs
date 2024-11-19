use anchor_lang::prelude::*;
use crate::{
    errors::*,
    states::config::Configs 
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{
        Mint, 
        TokenAccount,
        TokenInterface,
        TransferChecked,
        transfer_checked,
        MintTo,
        mint_to
    }
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
        init_if_needed,
        payer = lp,
        seeds = [b"mint_lp",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    
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
        associated_token::mint = mint_lp,
        associated_token::authority = lp
    )]  
    pub lp_account_mint: Box<InterfaceAccount<'info, TokenAccount>>,
    
    #[account(
        seeds = [b"config", config.seeds.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Configs>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, is_a: bool, amount: u64) -> Result<()> {

        let mint;

        let (from, to) = if is_a {
            mint = self.mint_a.clone();
            (
                self.lp_account_a.to_account_info(),
                self.vault_a.to_account_info()
            )
        } else {
            mint = self.mint_b.clone();
            (
                self.lp_account_b.to_account_info(),
                self.vault_b.to_account_info()
            )
        };

        let cpi_accounts = TransferChecked {
            authority: self.lp.to_account_info(),
            from,
            mint: mint.to_account_info(),
            to
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        
        transfer_checked(cpi_context, amount, 6)

    }

    pub fn mint_lp(&mut self, amount: u64) -> Result<()> {

        let cpi_accounts = MintTo{
            authority: self.auth.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            to: self.lp_account_mint.to_account_info()
        };


        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];


        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

        mint_to(ctx, amount)
    }

    

}

