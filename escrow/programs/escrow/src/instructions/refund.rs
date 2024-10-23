use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount}, token_interface::TokenInterface};

use crate::states::escrow::Escrow;


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority= maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref()],
        bump,
        close = maker,
        has_one = mint_a
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    pub vault: SystemAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Refund<'info> {
    pub fn refund_and_clean_vault(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        
    }
}