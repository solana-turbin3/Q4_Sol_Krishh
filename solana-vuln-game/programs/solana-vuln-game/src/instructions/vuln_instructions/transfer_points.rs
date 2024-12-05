use anchor_lang::prelude::*;
use crate::states::user::User;

#[derive(Accounts)]
#[instruction(id_sender: u32, id_receiver: u32)]
pub struct TransferPoints<'info> {
    #[account(
        seeds = [b"user", id_sender.to_le_bytes().as_ref()], 
        bump
    )]
    pub sender: Account<'info, User>,
    #[account(
        seeds = [b"user", id_receiver.to_le_bytes().as_ref()], 
        bump
    )]
    pub receiver: Account<'info, User>,
    /// CHECK: THis is part of the vuln. programs
    #[account(mut)]
    pub signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> TransferPoints<'info> {
    pub fn transfer_points(&mut self, _id_sender: u32, _id_receiver: u32, amount: u16) -> Result<()> {
        let sender = &mut self.sender;
        let receiver = &mut self.receiver;

        if sender.points < amount {
            return err!(Error::NotEnoughPoints);
        }
        sender.points -= amount;
        receiver.points += amount;
        msg!("Transferred {} points", amount);
        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("No Enough Points")]
    NotEnoughPoints
}