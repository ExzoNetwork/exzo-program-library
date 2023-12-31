//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RelyingParty {
    /// Incorrect authority provided on update or delete
    #[error("Incorrect authority provided on update or delete")]
    IncorrectAuthority,

    /// Calculation overflow
    #[error("Calculation overflow")]
    Overflow,

    /// Relying Party derived address does not match with generated address
    #[error("Invalid Relying Party derived address provided")]
    InvalidRelyingPartyAddress,

    /// Account is not rent-exempt
    #[error("AccountNotRentExempt")]
    AccountNotRentExempt,

    /// Account icon CID invalid https://docs.ipfs.io/concepts/content-addressing/
    #[error("InvalidIconCID")]
    InvalidIconCID,
}
impl From<RelyingParty> for ProgramError {
    fn from(e: RelyingParty) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for RelyingParty {
    fn type_of() -> &'static str {
        "Record Error"
    }
}
