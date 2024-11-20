use anchor_lang::{prelude::*, system_program::Transfer};

use anchor_spl::{
    token_interface::{
        TokenAccount,
        TokenInterface,
        Mint,
        TransferChecked,
        transfer_checked
    },
    associated_token::AssociatedToken
};
use crate::{
    states::config::Configs,
    errors::AmmError,
};


#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
        payer = user,
        associated_token::mint = mint_a,
        associated_token::authority = user
    )]
    pub user_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user, 
        associated_token::mint = mint_b,
        associated_token::authority = user
    )]  
    pub user_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [b"config", config.seeds.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Configs>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Swap<'info> {
    pub fn deposit_token(&mut self, is_a: bool, amount: u64) -> Result<()> {

        let mint;

        let (from, to) = if is_a {
            mint = self.mint_a.clone();
            (
                self.user_account_a.to_account_info(),
                self.vault_a.to_account_info()
            )
        } else {
            mint = self.mint_b.clone();
            (
                self.user_account_b.to_account_info(),
                self.vault_b.to_account_info()
            )
        };

        let cpi_accounts = TransferChecked {
            authority: self.user.to_account_info(),
            from,
            mint: mint.to_account_info(),
            to
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        
        transfer_checked(cpi_context, amount, 6)
    }

    pub fn withdraw(&mut self, is_a: bool, amount: u64)-> Result<()> {

        let mint;

        let (from, to) = if is_a {
            mint = self.mint_b.clone();

            (
                self.vault_b.to_account_info(),
                self.user_account_b.to_account_info(),
            )
        } else {
            mint = self.mint_a.clone();

            (
                self.vault_a.to_account_info(),
                self.user_account_a.to_account_info(),
            )
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from,
            to,
            mint: mint.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            cpi_accounts, 
            signer_seeds
        );

        transfer_checked(cpi_context, amount,6)

    }


}