use anchor_lang::prelude::*;

#[error_code]
pub enum LedgerError {
    #[msg("unauthorized")]
    Unauthorized,

    #[msg("insufficient balance")]
    InsufficientBalance,

    #[msg("overflow occured")]
    Overflow,
}