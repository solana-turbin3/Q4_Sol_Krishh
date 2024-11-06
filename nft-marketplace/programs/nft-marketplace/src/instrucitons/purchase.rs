use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::states::{listing::Listing, marketplace::MarketPlace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(
        mut, 
        seeds = [b"maekteplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed ,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [marketplace.key().as_ref(), maker.key().as_ref() ],
        bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.reward_bump
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"treasury",  marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System> 
}


impl<'info> Purchase<'info> {
    pub fn send_sol(&self) -> Result<()> {

        let marketplace_fee = (self.marketplace.fee as u64) 
        .checked_mul(self.listing.price)
        .unwrap()
        .checked_div(10000_u64)
        .unwrap();

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info()
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);


        transfer(cpi_context, amount)?;

        let cpi_acc_tresury = Transfer{
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let cpi_program = self.system_program.to_account_info();

        let cpi_context_treasury = CpiContext::new(cpi_program, cpi_acc_tresury);

        transfer(cpi_context_treasury, marketplace_fee)
    }

    pub fn send_nft(&self) -> Result<()> {
        
        let listing_seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        
        let signer_seeds = &[&listing_seeds[..]];

        let token_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            token_program,
            cpi_accounts,
            signer_seeds
        );
        transfer_checked(cpi_context, 1, self.maker_mint.decimals)
    }

    pub fn close_vault(&self) -> Result<()> {

        let listing_seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        
        let signer_seeds = &[&listing_seeds[..]];

        let token_program = self.token_program.to_account_info();

        let close_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            token_program,
            close_accounts, 
            signer_seeds);

        close_account(cpi_context)
    }

}