use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount, TransferChecked}, token_interface::{TokenInterface, transfer_checked}};

use crate::states::escrow::Escrow;

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        associated_token::token_program = token_program,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, 
        seeds = [b"escrow", maker.key().as_ref()],
        space =  Escrow::INIT_SPACE + 8,
        bump,
        payer = maker
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init, 
        payer= maker,
        associated_token::authority = escrow,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program
    )]
    pub vault: SystemAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, bumps: MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            bump: bumps.escrow,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key()
        });
        
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64)-> Result<()> {

        let transfer_accounts = TransferChecked {
            authority: self.maker.to_account_info(),
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        
        transfer_checked(cpi_context, deposit, self.mint_a.decimals)

        Ok(())
    }
}