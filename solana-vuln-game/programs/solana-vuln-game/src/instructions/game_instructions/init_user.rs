use anchor_lang::prelude::*;
use crate::states::game_state::UserState;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 1,
        seeds = [b"user_account", user.key.as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>
}


impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps) ->  Result<()> {

        // Create a new user in the game
        self.user_account.set_inner( UserState {
            points: 0,
            user_bump: bumps.user_account,
        });

        Ok(())
    }
}