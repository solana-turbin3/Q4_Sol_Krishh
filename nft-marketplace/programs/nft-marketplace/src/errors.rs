use anchor_lang::prelude::*;

#[error_code]
pub enum ERRORS {
    #[msg("Invalid name length")]
    INVALID_STRING_LENGTH,
}
