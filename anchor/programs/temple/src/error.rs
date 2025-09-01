/// Errors that may be returned by the TokenSwap program.
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Input account owner is not the program address")]
    InvalidOwner,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid incense ID")]
    InvalidIncenseId,
    #[msg("Insufficient SOL balance to pay for incense")]
    InsufficientSolBalance,
    #[msg("Temple treasury account mismatch")]
    InvalidTempleTreasury,
}
