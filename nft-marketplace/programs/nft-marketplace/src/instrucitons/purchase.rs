use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

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

    

}