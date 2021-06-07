use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum RevenueSharingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Withdraw Limit Exceeded")]
    WithdrawLimitExceeded
}

impl From<RevenueSharingError> for ProgramError {
    fn from(err: RevenueSharingError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
