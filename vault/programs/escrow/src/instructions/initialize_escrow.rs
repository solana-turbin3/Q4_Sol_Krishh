use anchor_lang::prelude::*;
use crate::states::EscrowState;


#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The user who's gonna pay for the account creation and txn fee
    
    #[account(
        init, 
        seeds=[b"escrow_pda", payer.key().as_ref()],
        bump,
        space = EscrowState::INIT_SPACE + 8,
        payer = payer
    )]
    pub escrow_pda: Account<'info, EscrowState>,

    #[account(
        seeds=[b"escrow_vault", payer.key().as_ref()],
        bump
    )]
    pub escrow_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl InitializeEscrow<'_> {
    pub fn new(&mut self, bumps: InitializeEscrowBumps) -> Result<()> {
        self.escrow_pda.set_inner(EscrowState{
            state_bump: bumps.escrow_pda, 
            vault_bump: bumps.escrow_vault
        });
        Ok(())
    }
}