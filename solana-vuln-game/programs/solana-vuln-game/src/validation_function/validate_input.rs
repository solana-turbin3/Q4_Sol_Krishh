use anchor_lang::prelude::*;
use anchor_spl::token_2022::{mint_to, MintTo};
pub use sha256::digest;

pub fn validate_inputs<'info>(
    sig: String,
    expect: &str,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    program: AccountInfo<'info>,
    bumps: u8,
) -> Result<()> {
    let hash = digest(sig);

    msg!("Hash:::{}", hash);

    require!(hash == expect, ErrorCode::InvalidInput);
    // Create the signer seeds for the platform payment mint's authority
    let mint_accounts = MintTo {
        authority,
        mint,
        to,
    };

    // Define the signer_seeds for the token
    let seeds = &[b"config".as_ref(), &[bumps]];

    let signer_seeds = &[&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(program, mint_accounts, signer_seeds);

    mint_to(cpi_context, 100)

}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid input")]
    InvalidInput,
}
