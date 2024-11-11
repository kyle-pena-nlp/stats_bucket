use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum Errors {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,
    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,
    #[error("Wrong bucket PDA")]
    WrongBucketPDA,
    #[error("Wrong source bucket PDA")]
    WrongSourceBucketPDA,
    #[error("Wrong target bucket PDA")]
    WrongTargetBucketPDA
}

impl PrintProgramError for Errors {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<Errors> for ProgramError {
    fn from(e: Errors) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for Errors {
    fn type_of() -> &'static str {
        "StatsBucket Error"
    }
}
