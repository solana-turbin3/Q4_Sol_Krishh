use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::states::EscrowState;

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds= [b"escrow_pda", payer.key().as_ref()],
        bump
    )]
    pub escrow_pda: Account<'info, EscrowState>,

    #[account(
        seeds= [b"escrow_vault", payer.key().as_ref()],
        bump
    )]
    pub escrow_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}


impl WithdrawAccounts<'_> {
    pub fn withdraw(&self, amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.escrow_vault.to_account_info(),
            to: self.payer.to_account_info()
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow_vault",
            self.payer.key.as_ref(),
            &[self.escrow_pda.vault_bump]
        ]];

        let cpi_context = CpiContext::new_with_signer(system_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)
    }
}