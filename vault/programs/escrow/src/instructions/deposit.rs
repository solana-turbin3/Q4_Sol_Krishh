use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::states::EscrowState;

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]   
    pub payer: Signer<'info>,
    #[account(
        seeds=[b"escrow_pda", payer.key().as_ref()],
        bump 
    )]
    pub escrow_pda: Account<'info, EscrowState>,

    #[account(
        seeds=[b"escrow_vault", payer.key().as_ref()],
        bump
    )] 
    pub escrow_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl DepositAccounts<'_> {
    pub fn deposit(&self, amount: u64)-> Result<()> {

        let system_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.payer.to_account_info(),
            to: self.escrow_vault.to_account_info()
        };

        let cpi_context = CpiContext::new(system_program,cpi_accounts);

        transfer(cpi_context, amount)
    }
}