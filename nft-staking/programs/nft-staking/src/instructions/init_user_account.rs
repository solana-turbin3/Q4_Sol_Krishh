use anchor_lang::prelude::*;

use crate::states::UserAccount::UserAccount;

#[derive(Accounts)]
pub struct InitUserAccount<'info>{
    /// This is the user initiating the account in the platform
    #[account(mut)]
    pub user: Signer<'info>, 
    /// PDA being initiated
    #[account(
        init, 
        seeds = [b"user_account".as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>
}

impl<'info> InitUserAccount<'info> {
    pub fn init_account(&mut self, bumps: &InitUserAccountBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount{
            bump: bumps.user_account,
            point: 0,
            amount_staked: 0
        });
        Ok(())
    }
}
