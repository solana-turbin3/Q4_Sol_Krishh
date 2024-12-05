pub use anchor_lang::prelude::*;
use crate::states::vul_state::User;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user_creator: Signer<'info>,
    #[account(
        init, 
        seeds = [b"user", user_creator.key().as_ref()],
        bump,
        space = 8 + 32 + (4 + 10) + 2,
        payer = user_creator
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateUser<'info> {
    pub fn init_user(&mut self, name: String) -> Result<()> {
        
        self.user.set_inner(User {
            name,
            owner: self.user_creator.key(),
            points: 1000
        });

        Ok(())
    }
}
