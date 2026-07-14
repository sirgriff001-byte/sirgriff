use solana_program::program_error::ProgramError;
use std::convert::From;

#[derive(Debug)]
pub enum StakingError {
    InvalidInstruction,
    NotRentExempt,
    InvalidOwner,
    InvalidMint,
    InvalidTokenAccount,
    InsufficientFunds,
    InvalidStakeAmount,
    LockPeriodNotExpired,
    NoStakedTokens,
    CalculationOverflow,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
