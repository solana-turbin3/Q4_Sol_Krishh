use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{
    Mint, TokenAccount, TokenInterface
}};
declare_id!("DEFuzL6ArEcszLSgy1pQBLSdyBd7BKR5CUdckq2RXn2A");

pub mod states;
pub mod instructions;
pub mod validation_function;


use states::game_state::{
    config::Config,
    user_state::UserState
};
use instructions::{
    game_instructions::init_config::*,
    game_instructions::init_user::*,
    vuln_instructions::create_user::*,
    vuln_instructions::transfer_points::*
};

pub use crate::validation_function::validate_input::validate_inputs;

#[program]
pub mod solana_vuln_game {
    use super::*;
    pub fn initialize_config(ctx: Context<InitConfig>) -> Result<()> {
        ctx.accounts.config_init(&ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn user_create(ctx: Context<CreateUser>, name: String) -> Result<()> {
        ctx.accounts.init_user(name)
    }

    pub fn transfer_point(ctx: Context<TransferPoints>, _id_sender: u32, _id_receiver: u32, amount: u16) -> Result<()> {
        ctx.accounts.transfer_points(_id_sender, _id_receiver, amount)
    }

    pub fn space_validation(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "c0754dd78e6e35d7b10126a84772c7aabd31e1ab08652581694f5a328e6f19bc", 
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.config.bump
        )?;

        ctx.accounts.user.points = ctx.accounts.user.points.checked_add(100).expect("Overflow occured");

        Ok(())
    }   
    pub fn input_validation(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "2bc76b3209c3ae44d538f880d94a290445bcf7e1a6df7b5db47e5ca9428b015c",
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.config.bump
        )?;

        ctx.accounts.user.points = ctx.accounts.user.points.checked_add(100).expect("Overflow occured");

        Ok(())
    }  

    pub fn arithmetic_underflow(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "92968f22c9b1fc76f6c7debdb837dd991f73379bf4bfa1c63daedce4a761b0e3", 
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.config.bump
        )?;

        ctx.accounts.user.points = ctx.accounts.user.points.checked_add(100).expect("Overflow occured");

        Ok(())
    }   

    pub fn arithmetic_overflow(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "e88bd622ecee27bc73d3dc61f49e5b989f617716359e659b14412e945e19563b", 
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.config.bump
        )?;

        ctx.accounts.user.points = ctx.accounts.user.points.checked_add(100).expect("Overflow occured");

        Ok(())
    }    

    pub fn program_id_verification(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "e3af90ff0ad9833f839289f890a1e6d81cd66e4cb419272ab21668d57133de57", 
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.config.bump
        )?;

        ctx.accounts.user.points = ctx.accounts.user.points.checked_add(100).expect("Overflow occured");

        Ok(())
    }  
}


#[derive(Accounts)]
pub struct PassTest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // The platform config
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    // The platform's mint
    #[account(
        mut,
        seeds = [b"payment_token", config.admin.as_ref(), config.key().as_ref()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user_account", signer.key().as_ref()],
        bump = user.user_bump
    )]
    pub user: Account<'info, UserState>,
    
    // The user pda with the mint
    #[account(
        init_if_needed,
        payer= signer, 
        associated_token::mint = mint_account,
        associated_token::authority = signer
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}